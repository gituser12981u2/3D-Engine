use super::buffer_manager::BufferManager;
use super::pipeline::create_render_pipeline;
use super::MacOSWindow;
use crate::renderer::backend::GraphicsBackend;
use crate::renderer::common::{RendererError, Vertex};
use cocoa::base::id as cocoa_id;
use core_graphics::display::CGSize;
use metal::MTLViewport;
use metal::{
    objc::{msg_send, sel, sel_impl},
    CommandQueue, Device, MetalLayer, RenderPipelineState,
};

pub struct MetalBackend {
    device: Device,
    command_queue: CommandQueue,
    pipeline_state: RenderPipelineState,
    buffer_manager: BufferManager,
    layer: MetalLayer,
    viewport: Viewport,
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
    pub fn new(window: &MacOSWindow) -> Result<Self, RendererError> {
        let device = Device::system_default().ok_or(RendererError::DeviceNotFound)?;
        let command_queue = device.new_command_queue();
        let pipeline_state = create_render_pipeline(&device)?;
        let buffer_manager = BufferManager::new();
        let metal_layer = Self::setup_metal_layer(&device, window.get_view())?;
        let (width, height) = window.get_size();

        Ok(MetalBackend {
            device,
            command_queue,
            pipeline_state,
            buffer_manager,
            layer: metal_layer,
            viewport: Viewport::new(0, 0, width, height),
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
        self.buffer_manager
            .update_vertex_buffer(&self.device, vertices)
    }

    fn update_index_buffer(&mut self, indices: &[u32]) -> Result<(), RendererError> {
        println!("Updating vertex buffer with {} indices", indices.len());
        self.buffer_manager
            .update_index_buffer(&self.device, indices)
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

        if let Some(vertex_buffer) = &self.buffer_manager.vertex_buffer {
            encoder.set_vertex_buffer(0, Some(vertex_buffer), 0);
            println!("Vertex buffer set");
        } else {
            println!("No vertex buffer available");
        }

        if let Some(index_buffer) = &self.buffer_manager.index_buffer {
            println!("Drawing indexed primitives");
            encoder.draw_indexed_primitives(
                metal::MTLPrimitiveType::Triangle,
                index_count as u64,
                metal::MTLIndexType::UInt32,
                index_buffer,
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

#[cfg(test)]
mod tests {
    use super::MetalBackend;
    use crate::renderer::backend::metal::pipeline::create_render_pipeline;
    use cocoa::base::id as cocoa_id;
    use metal::foreign_types::ForeignType;
    use metal::{Device, MTLPixelFormat};
    use objc::{class, msg_send, sel, sel_impl};

    #[test]
    fn test_setup_metal_layer() {
        let device = Device::system_default().unwrap();
        let view: cocoa_id = unsafe { msg_send![class!(NSView), new] };

        let layer_result = MetalBackend::setup_metal_layer(&device, view);
        assert!(layer_result.is_ok());

        let layer = layer_result.unwrap();

        unsafe {
            let layer_device: *mut std::os::raw::c_void = msg_send![layer.as_ref(), device];
            assert_eq!(layer_device, device.as_ptr() as *mut std::os::raw::c_void);

            let pixel_format: MTLPixelFormat = msg_send![layer.as_ref(), pixelFormat];
            assert_eq!(pixel_format, MTLPixelFormat::BGRA8Unorm);

            let presents_with_transaction: bool =
                msg_send![layer.as_ref(), presentsWithTransaction];
            assert_eq!(presents_with_transaction, false);

            // Check if the layer is properly set on the view
            let view_layer: cocoa_id = msg_send![view, layer];
            assert_eq!(view_layer as *const _, layer.as_ptr() as *const _);

            let wants_layer: bool = msg_send![view, wantsLayer];
            assert_eq!(wants_layer, true);
        }
    }

    #[test]
    fn test_create_render_pipeline() {
        let device = Device::system_default().expect("No Metal device found");
        let result = create_render_pipeline(&device);
        assert!(
            result.is_ok(),
            "Failed to create render pipeline: {:?}",
            result.err()
        );
    }
}
