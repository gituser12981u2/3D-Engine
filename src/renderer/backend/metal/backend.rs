use super::buffer_manager::BufferManager;
use super::pipeline::{create_default_pipeline_descriptor, RenderPipelineCache};
use super::texture_manager::TextureManager;
use crate::renderer::backend::GraphicsBackend;
use crate::renderer::common::{BackendDrawCommand, RendererError, TextureId, Uniforms, Vertex};
use crate::renderer::InstanceData;
use cocoa::base::id as cocoa_id;
use core_graphics::display::CGSize;
use log::{debug, info, trace, warn};
use metal::{
    foreign_types::ForeignTypeRef, BufferRef, DepthStencilState, MTLRegion, MTLViewport,
    MetalDrawableRef, RenderCommandEncoderRef, RenderPassDescriptorRef, RenderPipelineDescriptor,
    TextureDescriptor, TextureRef,
};
use metal::{
    objc::{msg_send, sel, sel_impl},
    CommandQueue, Device, MetalLayer,
};
use raw_window_handle::HasWindowHandle;
use winit::window::Window;

/// Represents the Metal backend for rendering.
pub struct MetalBackend {
    device: Device,
    command_queue: CommandQueue,
    render_pipeline_cache: RenderPipelineCache,
    buffer_manager: BufferManager,
    texture_manager: TextureManager,
    layer: MetalLayer,
    depth_stencil_state: DepthStencilState,
    wireframe_mode: bool,
    // pipeline_cache: HashMap<PipelineType, RenderPipelineId>,
    // current_pipeline_type: PipelineType,
}

impl MetalBackend {
    /// Creates a new MetalBackend instance.
    ///
    /// # Arguments
    ///
    /// * `window` - The window to which the Metal layer will be attached.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the MetalBackend instance or a RendererError.
    pub fn new(window: &Window) -> Result<Self, RendererError> {
        let device = Device::system_default().ok_or(RendererError::DeviceNotFound)?;
        info!("Metal device initialized");

        let command_queue = device.new_command_queue();
        let mut render_pipeline_cache = RenderPipelineCache::new(&device)?;
        let buffer_manager = BufferManager::new(&device)?;
        let texture_manager = TextureManager::new(&device);

        let (default_pipeline_descriptor, depth_stencil_state) =
            create_default_pipeline_descriptor(&device)?;
        render_pipeline_cache.create_pipeline_state(&default_pipeline_descriptor)?;

        let layer = Self::create_metal_layer_for_window(window, &device)?;

        info!("MetalBackend initialized successfully");
        Ok(MetalBackend {
            device,
            command_queue,
            render_pipeline_cache,
            buffer_manager,
            texture_manager,
            layer,
            depth_stencil_state,
            wireframe_mode: false,
        })
    }

    /// Creates a Metal Layer for the given window.
    ///
    /// # Arguments
    ///
    /// * `window` - The window to which the Metal layer will be attached.
    /// * `device` - The Metal device.
    ///
    /// # Returns
    ///
    /// Returns a Result containing the MetalLayer or a RendererError.
    fn create_metal_layer_for_window(
        window: &Window,
        device: &Device,
    ) -> Result<MetalLayer, RendererError> {
        match window.window_handle()?.as_raw() {
            raw_window_handle::RawWindowHandle::AppKit(handle) => {
                let ns_view = handle.ns_view.as_ptr() as cocoa_id;
                let layer = MetalLayer::new();

                layer.set_device(device);
                layer.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
                layer.set_presents_with_transaction(false);

                let size = window.inner_size();
                let scale_factor = window.scale_factor();

                let physical_metal_size = CGSize::new(size.width as f64, size.height as f64);
                layer.set_drawable_size(physical_metal_size);

                debug!(
                    "Setting Metal layer drawable size to: {:?} and scale factor is: {:?}",
                    physical_metal_size, scale_factor
                );

                unsafe {
                    let () = msg_send![ns_view, setLayer:layer.as_ref()];
                    let () = msg_send![ns_view, setWantsLayer:true];
                }

                Ok(layer)
            }
            _ => {
                warn!("Unsupported platform for Metal rendering");
                Err(RendererError::UnsupportedPlatform)
            }
        }
    }

    /// Creates a viewport for the given drawable
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

    /// Returns a reference to the Metal device.
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Toggles the wireframe mode.
    pub fn toggle_wireframe_mode(&mut self) {
        self.wireframe_mode = !self.wireframe_mode;
        info!("Wireframe mode toggled: {}", self.wireframe_mode);
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

        // Update depth texture if needed
        let texture_size = CGSize::new(texture.width() as f64, texture.height() as f64);
        self.buffer_manager.ensure_depth_texture(texture_size);

        let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
        color_attachment.set_texture(Some(texture));
        color_attachment.set_load_action(metal::MTLLoadAction::Clear);
        color_attachment.set_clear_color(metal::MTLClearColor::new(0.1, 0.1, 0.1, 1.0)); // Dark gray background
        color_attachment.set_store_action(metal::MTLStoreAction::Store);

        // Set up depth attachment
        let depth_attachment = descriptor.depth_attachment().unwrap();
        depth_attachment.set_texture(
            self.buffer_manager
                .depth_texture
                .as_ref()
                .map(|t| t as &TextureRef),
        );
        depth_attachment.set_load_action(metal::MTLLoadAction::Clear);
        depth_attachment.set_clear_depth(1.0);
        depth_attachment.set_store_action(metal::MTLStoreAction::Store);

        let command_buffer = self.command_queue.new_command_buffer();
        let encoder = command_buffer.new_render_command_encoder(descriptor);

        let viewport = self.create_viewport(drawable);
        let mut render_pass = RenderPass::new(encoder, viewport);

        render_pass.set_depth_stencil_state(&self.depth_stencil_state);
        render_pass.set_wireframe_mode(self.wireframe_mode);

        // Set the pipeline state
        let pipeline_state = self
            .render_pipeline_cache
            .get_pipeline_state()
            .ok_or(RendererError::InvalidPipelineId)?;
        render_pass.set_pipeline(pipeline_state);

        // Set vertex and uniform buffers
        render_pass.set_vertex_buffer(0, Some(&self.buffer_manager.vertex_buffer), 0);
        render_pass.set_vertex_buffer(1, Some(&self.buffer_manager.uniform_buffer), 0);
        trace!("Vertex and uniform buffers set");

        render_pass.draw(draw_command, &self.buffer_manager);
        render_pass.end();

        command_buffer.present_drawable(drawable);
        command_buffer.commit();

        Ok(())
    }

    fn update_vertex_buffer(&mut self, vertices: &[Vertex]) -> Result<(), RendererError> {
        trace!("Updating vertex buffer with {} vertices", vertices.len());
        self.buffer_manager.update_vertex_buffer(vertices)
    }

    fn update_index_buffer(&mut self, indices: &[u32]) -> Result<(), RendererError> {
        trace!("Updating index buffer with {} indices", indices.len());
        self.buffer_manager.update_index_buffer(indices)
    }

    fn update_uniform_buffer(&mut self, uniforms: &Uniforms) -> Result<(), RendererError> {
        trace!("Updating uniform buffer");
        self.buffer_manager.update_uniform_buffer(uniforms)
    }

    fn update_instance_buffer(&mut self, instances: &[InstanceData]) -> Result<(), RendererError> {
        trace!(
            "Updating instance buffer with {} instances",
            instances.len()
        );
        self.buffer_manager.update_instance_buffer(instances)
    }

    fn create_texture(&mut self, descriptor: &TextureDescriptor) -> TextureId {
        debug!("Creating new texture");
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
        trace!("Updating texture: {:?}", id);
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
    ) -> Result<(), RendererError> {
        debug!("Creating new render pipeline state");
        self.render_pipeline_cache.create_pipeline_state(descriptor)
    }

    // TODO: Use render pass for batch calling
    #[allow(unused_variables)]
    fn render_pass(&mut self, descriptor: &RenderPassDescriptorRef) -> Result<(), RendererError> {
        let drawable = self
            .layer
            .next_drawable()
            .ok_or(RendererError::DrawFailed("No next drawable".to_string()))?;

        // let command_buffer = self.command_queue.new_command_buffer();
        // let encoder = command_buffer.new_render_command_encoder(descriptor);

        let viewport = MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: drawable.texture().width() as f64,
            height: drawable.texture().height() as f64,
            znear: 0.0,
            zfar: 1.0,
        };

        trace!("Created render pass with viewport: {:?}", viewport);
        // Ok(RenderPass::new(encoder, viewport))
        Ok(())
    }
}

/// Represents a render pass in the Metal backend.
pub struct RenderPass<'a> {
    encoder: &'a RenderCommandEncoderRef,
    viewport: MTLViewport,
}

impl<'a> RenderPass<'a> {
    /// Creates a new RenderPass instance.
    pub fn new(encoder: &'a RenderCommandEncoderRef, viewport: MTLViewport) -> Self {
        RenderPass { encoder, viewport }
    }

    /// Sets the render pipeline state.
    pub fn set_pipeline(&mut self, pipeline: &metal::RenderPipelineState) {
        self.encoder.set_render_pipeline_state(pipeline);
    }

    /// Sets a vertex buffer.
    pub fn set_vertex_buffer(&self, index: u64, buffer: Option<&BufferRef>, offset: u64) {
        self.encoder.set_vertex_buffer(index, buffer, offset);
    }

    /// Sets the depth stencil state.
    pub fn set_depth_stencil_state(&mut self, state: &DepthStencilState) {
        self.encoder.set_depth_stencil_state(state);
    }

    /// Sets the wireframe mode for rendering.
    pub fn set_wireframe_mode(&mut self, wireframe: bool) {
        unsafe {
            let raw_encoder = self.encoder.as_ptr();
            let () = msg_send![raw_encoder, setTriangleFillMode:
            if wireframe {
                metal::MTLTriangleFillMode::Lines
            } else {
                metal::MTLTriangleFillMode::Fill
            }
            ];
        }
        trace!("Wireframe mode set to: {wireframe}");
    }

    /// Executes the draw command.
    fn draw(&mut self, draw_command: BackendDrawCommand, buffer_manager: &BufferManager) {
        self.encoder.set_viewport(self.viewport);

        match draw_command {
            BackendDrawCommand::Basic {
                primitive_type,
                vertex_start,
                vertex_count,
            } => {
                trace!(
                    "Drawing basic primitives: type={:?}, start={}, count={}",
                    primitive_type,
                    vertex_start,
                    vertex_count
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
                trace!(
                    "Drawing indexed primitives: type={:?}, count={}, index_type={:?}, offset={}",
                    primitive_type,
                    index_count,
                    index_type,
                    index_buffer_offset
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
                trace!(
                    "Drawing instanced primitives: type={:?}, start={}, count={}, instances={}",
                    primitive_type,
                    vertex_start,
                    vertex_count,
                    instance_count
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
                trace!("Drawing indexed instanced primitives: type={:?}, count={}, index_type={:?}, offset={}, instances={}", 
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

#[cfg(test)]
mod tests {
    use super::RenderPass;
    use metal::{Device, MTLViewport};

    #[test]
    #[cfg_attr(feature = "skip_metal_tests", ignore)]
    // Skip test because MTLViewport cannot be made in CI's headless macOS environment
    fn test_render_pass_creation() {
        let device = Device::system_default().expect("No Metal device found");
        let command_queue = device.new_command_queue();
        let command_buffer = command_queue.new_command_buffer();
        let descriptor = metal::RenderPassDescriptor::new();
        let encoder = command_buffer.new_render_command_encoder(&descriptor);
        let viewport = MTLViewport {
            originX: 0.0,
            originY: 0.0,
            width: 800.0,
            height: 600.0,
            znear: 0.0,
            zfar: 1.0,
        };

        let render_pass = RenderPass::new(encoder, viewport);
        assert_eq!(render_pass.viewport.width, 800.0);
        assert_eq!(render_pass.viewport.height, 600.0);
    }
}
