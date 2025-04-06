//! Metal rendering pipeline management module.
//!
//! This module provides functionality to create and manage Metal rendering pipelines,
//! including pipeline state caching and default pipeline descriptor creation.

use super::shader_library::{ShaderLibrary, ShaderLoadOptions};
use crate::renderer::RendererError;
use log::{debug, error, info, trace};
use metal::{
    DepthStencilDescriptor, DepthStencilState, Device, MTLPixelFormat, MTLVertexFormat,
    RenderPipelineDescriptor, RenderPipelineState,
};

/// Manages the caching of Metal render pipeline states.
pub struct RenderPipelineCache {
    device: Device,
    shader_library: ShaderLibrary,
    pipeline_state: Option<RenderPipelineState>,
}

impl RenderPipelineCache {
    /// Creates a new `RenderPipelineCache` instance.
    ///
    /// # Arguments
    ///
    /// * `device` - A reference to the Metal device.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `RenderPipelineCache` or a `RendererError`.
    pub fn new(device: &Device, shader_options: &ShaderLoadOptions) -> Result<Self, RendererError> {
        let shader_library = ShaderLibrary::new(device, shader_options)?;

        Ok(RenderPipelineCache {
            device: device.clone(),
            shader_library,
            pipeline_state: None,
        })
    }

    /// Creates and caches a new pipeline state.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - A reference to the `RenderPipelineDescriptor`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `RendererError`.
    pub fn create_pipeline_state(
        &mut self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<(), RendererError> {
        debug!("Creating new pipeline state");

        let pipeline_state = self
            .device
            .new_render_pipeline_state(descriptor)
            .map_err(|e| {
                error!("Failed to create pipeline state: {e}");
                RendererError::PipelineCreationFailed(e.to_string())
            })?;

        self.pipeline_state = Some(pipeline_state);
        info!("New pipeline state created and cached");
        Ok(())
    }

    /// Retrieves the cached pipeline state.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `RenderPipelineState` if available.
    pub fn get_pipeline_state(&self) -> Option<&RenderPipelineState> {
        self.pipeline_state.as_ref()
    }

    /// Returns a reference to the shader library
    pub fn get_shader_library(&self) -> &ShaderLibrary {
        &self.shader_library
    }
}

pub fn create_default_pipeline_state(
    device: &Device,
    shader_options: &ShaderLoadOptions,
) -> Result<(RenderPipelineCache, DepthStencilState), RendererError> {
    debug!("Creating default pipeline setup");

    let mut pipeline_cache = RenderPipelineCache::new(device, shader_options)?;

    let shader_library = pipeline_cache.get_shader_library();

    // Default setup: non-instanced, using vertex colors
    let (vertex_function, fragment_function) = shader_library.get_shader_functions(false, true)?;

    // Create a pipeline descriptor
    let pipeline_descriptor = create_pipeline_descriptor(&vertex_function, &fragment_function);
    setup_vertex_descriptor(&pipeline_descriptor);

    // Create the pipeline state
    pipeline_cache.create_pipeline_state(&pipeline_descriptor)?;

    let depth_stencil_state = create_depth_stencil_state(device);

    info!("Default render pipeline state created");
    Ok((pipeline_cache, depth_stencil_state))
}

fn create_pipeline_descriptor(
    vertex_function: &metal::Function,
    fragment_function: &metal::Function,
) -> RenderPipelineDescriptor {
    debug!("Creating pipeline descriptor");
    let pipeline_descriptor = metal::RenderPipelineDescriptor::new();
    pipeline_descriptor.set_vertex_function(Some(vertex_function));
    pipeline_descriptor.set_fragment_function(Some(fragment_function));

    // Setup color attachments
    let attachment = pipeline_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap();
    attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    // Add depth attachment
    pipeline_descriptor.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);

    pipeline_descriptor
}

fn create_depth_stencil_state(device: &Device) -> DepthStencilState {
    debug!("Creating depth stencil state");

    // Enable depth testing
    let depth_stencil_descriptor = DepthStencilDescriptor::new();
    depth_stencil_descriptor.set_depth_compare_function(metal::MTLCompareFunction::Less);
    depth_stencil_descriptor.set_depth_write_enabled(true);

    device.new_depth_stencil_state(&depth_stencil_descriptor)
}

fn setup_vertex_descriptor(pipeline_descriptor: &RenderPipelineDescriptor) {
    debug!("Setting up vertex descriptor");
    let vertex_descriptor = metal::VertexDescriptor::new();

    // Position attribute
    let position_attr = vertex_descriptor.attributes().object_at(0).unwrap();
    position_attr.set_format(MTLVertexFormat::Float3);
    position_attr.set_offset(0);
    position_attr.set_buffer_index(0);

    // Color attribute
    let color_attr = vertex_descriptor.attributes().object_at(1).unwrap();
    color_attr.set_format(MTLVertexFormat::Float4);
    color_attr.set_offset(12); // 3 floats
    color_attr.set_buffer_index(0);

    // Vertex buffer layout
    vertex_descriptor
        .layouts()
        .object_at(0)
        .unwrap()
        .set_stride(28); // 7 floats per vertex (3 for position, 4 for color)

    trace!(
        "Vertex descriptor details:\n
            Position: format={:?}, offset={}, buffer_index={}\n 
            Color: format={:?}, offset={}, buffer_index={}\n,
            Stride: {}",
        position_attr.format(),
        position_attr.offset(),
        position_attr.buffer_index(),
        color_attr.format(),
        color_attr.offset(),
        color_attr.buffer_index(),
        vertex_descriptor.layouts().object_at(0).unwrap().stride()
    );

    pipeline_descriptor.set_vertex_descriptor(Some(vertex_descriptor));

    debug!("Vertex descriptor set up successfully");
}

#[cfg(test)]
mod tests {
    use metal::Device;

    use crate::renderer::backend::metal::{
        pipeline::create_default_pipeline_state, ShaderLoadOptions,
    };

    #[test]
    fn test_create_default_pipeline_descriptor() {
        let device = Device::system_default().expect("No Metal device found");
        let shader_options = ShaderLoadOptions::default();
        let result = create_default_pipeline_state(&device, &shader_options);
        assert!(
            result.is_ok(),
            "Failed to create render pipeline: {:?}",
            result.err()
        );
    }
}
