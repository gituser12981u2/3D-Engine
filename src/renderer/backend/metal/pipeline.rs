//! Metal rendering pipeline management module.
//!
//! This module provides functionality to create and manage Metal rendering pipelines,
//! including pipeline state caching and default pipeline descriptor creation.

use crate::renderer::RendererError;
use log::{debug, error, info, trace};
use metal::{
    DepthStencilDescriptor, DepthStencilState, Device, MTLDataType, MTLPixelFormat,
    MTLVertexFormat, RenderPipelineDescriptor, RenderPipelineState,
};
use std::ffi::c_void;

/// Manages the caching of Metal render pipeline states.
pub struct RenderPipelineCache {
    device: Device,
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
    pub fn new(device: &Device) -> Result<Self, RendererError> {
        Ok(RenderPipelineCache {
            device: device.clone(),
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
}

/// Creates a default render pipeline descriptor and a depth stencil state.
///
/// # Arguments
///
/// * `device` - A reference to the Metal device.
///
/// # Returns
///
/// A `Result` containing a tuple of `(RenderPipelineDescriptor, DepthStencilState)` or a `RendererError`.
pub fn create_default_pipeline_descriptor(
    device: &Device,
) -> Result<(RenderPipelineDescriptor, DepthStencilState), RendererError> {
    debug!("Creating default pipeline descriptor");

    let library = load_metal_shader_library(device)?;
    let (vertex_function, fragment_function) = create_shader_functions(&library)?;
    let pipeline_descriptor = create_pipeline_descriptor(&vertex_function, &fragment_function);
    let depth_stencil_state = create_depth_stencil_state(device);

    setup_vertex_descriptor(&pipeline_descriptor);

    // Create the render pipeline state
    info!("Render pipeline state created");
    Ok((pipeline_descriptor, depth_stencil_state))
}

fn load_metal_shader_library(device: &Device) -> Result<metal::Library, RendererError> {
    debug!("Loading pre-compiled shaders");

    // Create compilation options
    let shader_lib_path = std::env::var("METAL_SHADER_LIB").map_err(|e| {
        error!("Failed to get shader lib path: {e}");
        RendererError::ShaderCompilationFailed(format!("Failed to get shader lib path: {e}"))
    })?;

    device.new_library_with_file(shader_lib_path).map_err(|e| {
        error!("Failed to load shader library: {e}");
        RendererError::ShaderCompilationFailed(format!("Failed to load shader library: {e}"))
    })
}

fn create_shader_functions(
    library: &metal::Library,
) -> Result<(metal::Function, metal::Function), RendererError> {
    debug!("Creating shader functions");

    // Create function constants for shader compilation
    // These constants are used to configure the shader behavior
    let function_constants = metal::FunctionConstantValues::new();

    // Set function constants for instancing and vertex color usage
    // These values correspond to the function_constant(0) and function_constant(1) in the shader code
    let is_instanced = false;
    let use_vertex_color = true;
    function_constants.set_constant_value_at_index(
        &is_instanced as *const bool as *const c_void,
        MTLDataType::Bool,
        0,
    );
    function_constants.set_constant_value_at_index(
        &use_vertex_color as *const bool as *const c_void,
        MTLDataType::Bool,
        1,
    );

    // Compile the vertex and fragment shaders
    let vertex_function = library
        .get_function("vertex_main", Some(function_constants))
        .map_err(|_| RendererError::ShaderFunctionNotFound("vertex_main".to_string()))?;
    let fragment_function = library
        .get_function("fragment_main", None)
        .map_err(|_| RendererError::ShaderFunctionNotFound("fragment_main".to_string()))?;

    let function_names: Vec<String> = library.function_names().into_iter().collect();
    debug!(
        "Shaders loaded successfully. Available functions:\n - {}",
        function_names.join("\n - ")
    );

    Ok((vertex_function, fragment_function))
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
    // TODO explore other depth compare function options
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
    use crate::renderer::backend::metal::pipeline::create_default_pipeline_descriptor;
    use metal::Device;

    #[test]
    fn test_create_default_pipeline_descriptor() {
        let device = Device::system_default().expect("No Metal device found");
        let result = create_default_pipeline_descriptor(&device);
        assert!(
            result.is_ok(),
            "Failed to create render pipeline: {:?}",
            result.err()
        );
    }
}
