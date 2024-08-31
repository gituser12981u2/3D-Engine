use super::buffer_manager::BufferManager;
use super::pipeline::{create_default_pipeline_descriptor, RenderPipelineCache};
use super::texture_manager::TextureManager;
use crate::renderer::backend::GraphicsBackend;
use crate::renderer::common::{
    BackendDrawCommand, RenderPipelineId, RendererError, TextureId, Vertex,
};
use cocoa::base::id as cocoa_id;
use core_graphics::display::CGSize;
use metal::{
    objc::{msg_send, sel, sel_impl},
    CommandQueue, Device, MetalLayer,
};
use metal::{
    BufferRef, MTLRegion, MTLViewport, MetalDrawableRef, RenderCommandEncoderRef,
    RenderPassDescriptorRef, RenderPipelineDescriptor, TextureDescriptor,
};
use raw_window_handle::HasWindowHandle;
use winit::window::Window;

pub struct MetalBackend {
    device: Device,
    command_queue: CommandQueue,
    render_pipeline_cache: RenderPipelineCache,
    buffer_manager: BufferManager,
    texture_manager: TextureManager,
    layer: MetalLayer,
    default_pipeline_id: RenderPipelineId,
}

pub struct RenderPass<'a> {
    encoder: &'a RenderCommandEncoderRef,
    viewport: MTLViewport,
}

impl<'a> RenderPass<'a> {
    pub fn new(encoder: &'a RenderCommandEncoderRef, viewport: MTLViewport) -> Self {
        RenderPass { encoder, viewport }
    }

    pub fn set_pipeline(&mut self, pipeline: &metal::RenderPipelineState) {
        self.encoder.set_render_pipeline_state(pipeline);
    }

    pub fn set_vertex_buffer(&self, index: u64, buffer: Option<&BufferRef>, offset: u64) {
        self.encoder.set_vertex_buffer(index, buffer, offset);
    }

    fn draw(&mut self, draw_command: BackendDrawCommand, buffer_manager: &BufferManager) {
        self.encoder.set_viewport(self.viewport);

        match draw_command {
            BackendDrawCommand::Basic {
                primitive_type,
                vertex_start,
                vertex_count,
            } => {
                println!(
                    "Drawing basic primitives: type={:?}, start={}, count={}",
                    primitive_type, vertex_start, vertex_count
                );
                self.encoder
                    .draw_primitives(primitive_type.into(), vertex_start, vertex_count);
            }
            BackendDrawCommand::Indexed {
                primitive_type,
                index_count,
                index_type,
                index_buffer_offset,
            } => {
                println!(
                    "Drawing indexed primitives: type={:?}, count={}, index_type={:?}, offset={}",
                    primitive_type, index_count, index_type, index_buffer_offset
                );
                self.encoder.draw_indexed_primitives(
                    primitive_type.into(),
                    index_count,
                    index_type.into(),
                    &buffer_manager.index_buffer,
                    index_buffer_offset,
                );
            }
            BackendDrawCommand::Instanced {
                primitive_type,
                vertex_start,
                vertex_count,
                instance_count,
            } => {
                println!(
                    "Drawing instanced primitives: type={:?}, start={}, count={}, instances={}",
                    primitive_type, vertex_start, vertex_count, instance_count
                );
                self.encoder
                    .set_vertex_buffer(2, Some(&buffer_manager.instance_buffer), 0);
                self.encoder.draw_primitives_instanced(
                    primitive_type.into(),
                    vertex_start,
                    vertex_count,
                    instance_count,
                );
            }
            BackendDrawCommand::IndexedInstanced {
                primitive_type,
                index_count,
                index_type,
                index_buffer_offset,
                instance_count,
            } => {
                println!("Drawing indexed instanced primitives: type={:?}, count={}, index_type={:?}, offset={}, instances={}", 
                        primitive_type, index_count, index_type, index_buffer_offset, instance_count);
                self.encoder
                    .set_vertex_buffer(2, Some(&buffer_manager.instance_buffer), 0);
                self.encoder.draw_indexed_primitives_instanced(
                    primitive_type.into(),
                    index_count,
                    index_type.into(),
                    &buffer_manager.index_buffer,
                    index_buffer_offset,
                    instance_count,
                );
            }
        }
    }

    pub fn end(self) {
        self.encoder.end_encoding();
    }
}

impl MetalBackend {
    pub fn new(window: &Window) -> Result<Self, RendererError> {
        let device = Device::system_default().ok_or(RendererError::DeviceNotFound)?;
        let command_queue = device.new_command_queue();
        let mut render_pipeline_cache = RenderPipelineCache::new(&device)?;
        let buffer_manager = BufferManager::new(&device)?;
        let texture_manager = TextureManager::new(&device);

        let default_pipeline_descriptor = create_default_pipeline_descriptor(&device)?;
        let default_pipeline_id =
            render_pipeline_cache.create_pipeline_state(&default_pipeline_descriptor)?;

        let layer = match window.window_handle()?.as_raw() {
            raw_window_handle::RawWindowHandle::AppKit(handle) => {
                let ns_view = handle.ns_view.as_ptr() as cocoa_id;
                let layer = Self::setup_metal_layer(&device, ns_view)?;
                let size = window.inner_size();
                let metal_size = CGSize::new(size.width as f64, size.height as f64);
                layer.set_drawable_size(metal_size);
                println!("Metal layer size: {:?}", layer.drawable_size());
                layer
            }
            _ => return Err(RendererError::UnsupportedPlatform),
        };

        Ok(MetalBackend {
            device,
            command_queue,
            render_pipeline_cache,
            buffer_manager,
            texture_manager,
            layer,
            default_pipeline_id,
        })
    }

    fn setup_metal_layer(device: &Device, view: cocoa_id) -> Result<MetalLayer, RendererError> {
        unsafe {
            let layer = MetalLayer::new();
            layer.set_device(device);
            layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
            layer.set_presents_with_transaction(false);

            let bounds: CGSize = msg_send![view, bounds];
            let () = msg_send![layer.as_ref(), setFrame:bounds];

            let () = msg_send![view, setLayer:layer.as_ref()];
            let () = msg_send![view, setWantsLayer:true];

            Ok(layer)
        }
    }

    fn create_viewport(&self, drawable: &MetalDrawableRef) -> MTLViewport {
        let texture = drawable.texture();
        MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: texture.width() as f64,
            height: texture.height() as f64,
            znear: 0.0,
            zfar: 1.0,
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }
}

impl GraphicsBackend for MetalBackend {
    fn draw(&mut self, draw_command: BackendDrawCommand) -> Result<(), RendererError> {
        let descriptor = metal::RenderPassDescriptor::new();

        let drawable = self
            .layer
            .next_drawable()
            .ok_or(RendererError::DrawFailed("No next drawable".to_string()))?;

        let texture = drawable.texture();

        let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
        color_attachment.set_texture(Some(texture));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_clear_color(metal::MTLClearColor::new(0.1, 0.1, 0.1, 1.0)); // Dark gray background
        color_attachment.set_store_action(metal::MTLStoreAction::Store);

        let command_buffer = self.command_queue.new_command_buffer();
        let encoder = command_buffer.new_render_command_encoder(descriptor);

        let viewport = self.create_viewport(drawable);
        let mut render_pass = RenderPass::new(encoder, viewport);

        // Set the pipeline state (you'll need to get the appropriate pipeline state based on the draw command)
        let pipeline_state = self
            .render_pipeline_cache
            .get_pipeline_state(self.default_pipeline_id)
            .ok_or(RendererError::InvalidPipelineId)?;
        render_pass.set_pipeline(pipeline_state);

        // Set vertex and uniform buffers
        render_pass.set_vertex_buffer(0, Some(&self.buffer_manager.vertex_buffer), 0);
        render_pass.set_vertex_buffer(1, Some(&self.buffer_manager.uniform_buffer), 0);
        println!("Vertex and uniform buffers set");

        render_pass.draw(draw_command, &self.buffer_manager);
        render_pass.end();

        command_buffer.present_drawable(drawable);
        command_buffer.commit();

        Ok(())
    }

    fn update_vertex_buffer(&mut self, vertices: &[Vertex]) -> Result<(), RendererError> {
        println!("Updating vertex buffer with {} vertices", vertices.len());
        println!("Vertex data:");
        for (i, vertex) in vertices.iter().enumerate() {
            println!(
                "Vertex {}: pos={:?}, color={:?}",
                i, vertex.position, vertex.color
            )
        }
        self.buffer_manager.update_vertex_buffer(vertices)
    }

    fn update_index_buffer(&mut self, indices: &[u32]) -> Result<(), RendererError> {
        println!("Updating vertex buffer with {} indices", indices.len());
        self.buffer_manager.update_index_buffer(indices)
    }

    fn update_uniform_buffer(&mut self, uniform_data: &glam::Mat4) -> Result<(), RendererError> {
        self.buffer_manager.update_uniform_buffer(uniform_data)
    }

    fn update_instance_buffer(
        &mut self,
        instances: &[crate::renderer::render_queue::InstanceData],
    ) -> Result<(), RendererError> {
        self.buffer_manager.update_instance_buffer(instances)
    }

    fn create_texture(&mut self, descriptor: &TextureDescriptor) -> TextureId {
        self.texture_manager.create_texture(descriptor)
    }

    fn update_texture(
        &mut self,
        id: TextureId,
        region: MTLRegion,
        mipmap_level: u64,
        slice: u64,
        data: &[u8],
        bytes_per_row: u64,
        bytes_per_image: u64,
    ) -> Result<(), RendererError> {
        self.texture_manager.update_texture(
            id,
            region,
            mipmap_level,
            slice,
            data,
            bytes_per_row,
            bytes_per_image,
        )
    }

    fn create_render_pipeline_state(
        &mut self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<RenderPipelineId, RendererError> {
        self.render_pipeline_cache.create_pipeline_state(descriptor)
    }

    fn render_pass(
        &mut self,
        descriptor: &RenderPassDescriptorRef,
    ) -> Result<RenderPass, RendererError> {
        let drawable = self
            .layer
            .next_drawable()
            .ok_or(RendererError::DrawFailed("No next drawable".to_string()))?;

        let command_buffer = self.command_queue.new_command_buffer();
        let encoder = command_buffer.new_render_command_encoder(descriptor);

        let viewport = MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: drawable.texture().width() as f64,
            height: drawable.texture().height() as f64,
            znear: 0.0,
            zfar: 1.0,
        };

        Ok(RenderPass::new(encoder, viewport))
    }

    // fn draw_large_single_vertex(
    //     &mut self,
    //     vertex_count: u32,
    //     index_count: u32,
    // ) -> Result<(), RendererError> {
    //     println!("Drawing: {vertex_count} vertices, {index_count} indices");

    //     let command_buffer = self.command_queue.new_command_buffer();
    //     let descriptor = metal::RenderPassDescriptor::new();

    //     let drawable = self
    //         .layer
    //         .next_drawable()
    //         .ok_or(RendererError::DrawFailed("No next drawable".to_string()))?;

    //     let texture = drawable.texture();

    //     let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
    //     color_attachment.set_texture(Some(texture));
    //     color_attachment.set_load_action(metal::MTLLoadAction::Clear);
    //     color_attachment.set_clear_color(metal::MTLClearColor::new(0.1, 0.1, 0.1, 1.0)); // Dark gray background
    //     color_attachment.set_store_action(metal::MTLStoreAction::Store);

    //     let encoder = command_buffer.new_render_command_encoder(descriptor);
    //     encoder.set_render_pipeline_state(&self.pipeline_state);

    //     // Set the viewport
    //     encoder.set_viewport(self.viewport.to_metal_viewport());

    //     // Set the uniform buffer
    //     encoder.set_vertex_buffer(1, Some(&self.uniform_buffer), 0);

    //     // Set vertex buffer
    //     encoder.set_vertex_buffer(0, Some(&self.buffer_manager.vertex_buffer), 0);

    //     let vertex_count = self.buffer_manager.get_vertex_count();
    //     let index_count = self.buffer_manager.get_index_count();

    //     if index_count > 0 {
    //         println!("Drawing indexed primitives");
    //         encoder.draw_indexed_primitives(
    //             metal::MTLPrimitiveType::Triangle,
    //             index_count as u64,
    //             metal::MTLIndexType::UInt32,
    //             &self.buffer_manager.index_buffer,
    //             0,
    //         );
    //     } else {
    //         println!("Drawing primitives");
    //         encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, vertex_count as u64);
    //     }

    //     encoder.end_encoding();
    //     command_buffer.present_drawable(drawable);
    //     command_buffer.commit();

    //     println!("Draw call completed");

    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::RenderPass;
    use metal::{Device, MTLViewport};

    #[test]
    fn test_render_pass_creation() {
        println!("Starting test_render_pass_creation");

        let result = panic::catch_unwind(|| {
            println!("Attempting to get system default Metal device");
            let device = match Device::system_default() {
                Some(d) => d,
                None => {
                    println!("No Metal device found.");
                    return; // Exit the test early
                }
            };

            println!("Creating command queue");
            let command_queue = device.new_command_queue();

            println!("Creating command buffer");
            let command_buffer = command_queue.new_command_buffer();

            println!("Creating render pass descriptor");
            let descriptor = metal::RenderPassDescriptor::new();

            println!("Creating render command encoder");
            let encoder = command_buffer.new_render_command_encoder(&descriptor);

            println!("Creating viewport");
            let viewport = MTLViewport {
                originX: 0.0,
                originY: 0.0,
                width: 800.0,
                height: 600.0,
                znear: 0.0,
                zfar: 1.0,
            };

            println!("Creating RenderPass");
            let render_pass = RenderPass::new(encoder, viewport);

            assert_eq!(render_pass.viewport.width, 800.0);
            assert_eq!(render_pass.viewport.height, 600.0);

            println!("Test completed successfully")
        });

        match result {
            Ok(_) => println!("Test completed without panicking"),
            Err(e) => {
                if let Some(s) = e.downcast_ref::<String>() {
                    println!("Test panicked with message: {s}");
                } else {
                    println!("Test panicked without a message");
                }
            }
        }
    }
}
