use super::buffer_manager::BufferManager;
use super::pipeline::create_render_pipeline;
use crate::renderer::backend::GraphicsBackend;
use crate::renderer::common::{RendererError, Vertex};
use cocoa::base::id as cocoa_id;
use core_graphics::display::CGSize;
use glam::Mat4;
use metal::{
    objc::{msg_send, sel, sel_impl},
    CommandQueue, Device, MetalLayer, RenderPipelineState,
};
use metal::{Buffer, MTLResourceOptions, MTLViewport};
use raw_window_handle::HasWindowHandle;
use winit::window::Window;

pub struct MetalBackend {
    command_queue: CommandQueue,
    pipeline_state: RenderPipelineState,
    buffer_manager: BufferManager,
    layer: MetalLayer,
    viewport: Viewport,
    uniform_buffer: Buffer,
}

struct Viewport {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Viewport {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Viewport {
            x,
            y,
            width,
            height,
        }
    }

    pub fn to_metal_viewport(&self) -> MTLViewport {
        MTLViewport {
            originX: self.x as f64,
            originY: self.y as f64,
            width: self.width as f64,
            height: self.height as f64,
            znear: 0.0,
            zfar: 1.0,
        }
    }
}

impl MetalBackend {
    pub fn new(window: &Window) -> Result<Self, RendererError> {
        let device = Device::system_default().ok_or(RendererError::DeviceNotFound)?;
        let command_queue = device.new_command_queue();
        let pipeline_state = create_render_pipeline(&device)?;
        let buffer_manager = BufferManager::new(&device)?;

        let metal_layer = match window.window_handle()?.as_raw() {
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

        let size = window.inner_size();
        let viewport = Viewport::new(0, 0, size.width, size.height);

        let uniform_buffer = device.new_buffer(
            std::mem::size_of::<Mat4>() as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        );

        Ok(MetalBackend {
            command_queue,
            pipeline_state,
            buffer_manager,
            layer: metal_layer,
            viewport,
            uniform_buffer,
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

    pub fn update_uniforms(&mut self, view_projection_matrix: Mat4) {
        let contents = self.uniform_buffer.contents() as *mut Mat4;
        unsafe {
            *contents = view_projection_matrix;
        }
        let range = metal::NSRange {
            location: 0,
            length: std::mem::size_of::<Mat4>() as u64,
        };
        self.uniform_buffer.did_modify_range(range);
    }
}

impl GraphicsBackend for MetalBackend {
    fn prepare_frame(&mut self) -> Result<(), RendererError> {
        // Any per-frame setup could go here
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

    fn draw(&mut self, vertex_count: u32, index_count: u32) -> Result<(), RendererError> {
        println!("Drawing: {vertex_count} vertices, {index_count} indices");

        let command_buffer = self.command_queue.new_command_buffer();
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

        let encoder = command_buffer.new_render_command_encoder(descriptor);
        encoder.set_render_pipeline_state(&self.pipeline_state);

        // Set the viewport
        encoder.set_viewport(self.viewport.to_metal_viewport());

        // Set the uniform buffer
        encoder.set_vertex_buffer(1, Some(&self.uniform_buffer), 0);

        encoder.set_vertex_buffer(0, Some(&self.buffer_manager.vertex_buffer), 0);

        let vertex_count = self.buffer_manager.get_vertex_count();
        let index_count = self.buffer_manager.get_index_count();

        if index_count > 0 {
            println!("Drawing indexed primitives");
            encoder.draw_indexed_primitives(
                metal::MTLPrimitiveType::Triangle,
                index_count as u64,
                metal::MTLIndexType::UInt32,
                &self.buffer_manager.index_buffer,
                0,
            );
        } else {
            println!("Drawing primitives");
            encoder.draw_primitives(metal::MTLPrimitiveType::Triangle, 0, vertex_count as u64);
        }

        encoder.end_encoding();
        command_buffer.present_drawable(drawable);
        command_buffer.commit();

        println!("Draw call completed");

        Ok(())
    }

    fn present_frame(&mut self) -> Result<(), RendererError> {
        // The actual submission is done in the draw method,
        // so this method is left empty or used for any post-frame operations
        Ok(())
    }
}
