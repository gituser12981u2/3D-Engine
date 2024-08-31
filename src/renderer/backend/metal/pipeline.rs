use crate::renderer::{common::RenderPipelineId, RendererError};
use metal::{Device, MTLDataType, MTLPixelFormat, RenderPipelineDescriptor, RenderPipelineState};
use std::{ffi::c_void, num::NonZeroU32};

pub struct RenderPipelineCache {
    device: Device,
    pipelines: Vec<Option<RenderPipelineState>>,
    free_ids: Vec<RenderPipelineId>,
}

impl RenderPipelineCache {
    pub fn new(device: &Device) -> Result<Self, RendererError> {
        Ok(RenderPipelineCache {
            device: device.clone(),
            pipelines: Vec::new(),
            free_ids: Vec::new(),
        })
    }

    pub fn create_pipeline_state(
        &mut self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<RenderPipelineId, RendererError> {
        let pipeline_state = self
            .device
            .new_render_pipeline_state(descriptor)
            .map_err(|e| RendererError::PipelineCreationFailed(e.to_string()))?;

        let id = if let Some(id) = self.free_ids.pop() {
            self.pipelines[id.0.get() as usize - 1] = Some(pipeline_state);
            id
        } else {
            let id = RenderPipelineId(
                NonZeroU32::new((self.pipelines.len() + 1) as u32)
                    .expect("Pipeline count overflow"),
            );
            self.pipelines.push(Some(pipeline_state));
            id
        };

        Ok(id)
    }

    pub fn get_pipeline_state(&self, id: RenderPipelineId) -> Option<&RenderPipelineState> {
        self.pipelines.get(id.0.get() as usize - 1)?.as_ref()
    }
}

pub fn create_default_pipeline_descriptor(
    device: &Device,
) -> Result<RenderPipelineDescriptor, RendererError> {
    println!("Loading pre-compiled shaders...");

    // Create compilation options
    let shader_lib_path = std::env::var("METAL_SHADER_LIB").map_err(|e| {
        RendererError::ShaderCompilationFailed(format!("Failed to get shader lib path: {}", e))
    })?;

    // Compile the library
    let library = device.new_library_with_file(shader_lib_path).map_err(|e| {
        println!("Shader compilation error: {:?}", e);
        RendererError::ShaderCompilationFailed(format!("Failed to load shader library: {}", e))
    })?;

    println!("Shaders loaded successfully. Available functions:");
    for function in library.function_names() {
        print!(" - {function}");
    }

    let function_constants = metal::FunctionConstantValues::new();
    let false_value: bool = false;
    let true_value: bool = true;
    function_constants.set_constant_value_at_index(
        &false_value as *const bool as *const c_void,
        MTLDataType::Bool,
        0,
    );
    function_constants.set_constant_value_at_index(
        &true_value as *const bool as *const c_void,
        MTLDataType::Bool,
        1,
    );

    let vertex_function = library
        .get_function("vertex_main", Some(function_constants))
        .map_err(|_| RendererError::ShaderFunctionNotFound("vertex_main".to_string()))?;
    let fragment_function = library
        .get_function("fragment_main", None)
        .map_err(|_| RendererError::ShaderFunctionNotFound("fragment_main".to_string()))?;

    // Create pipeline descriptor
    let pipeline_descriptor = metal::RenderPipelineDescriptor::new();
    pipeline_descriptor.set_vertex_function(Some(&vertex_function));
    pipeline_descriptor.set_fragment_function(Some(&fragment_function));

    // Setup color attachments
    let attachment = pipeline_descriptor
        .color_attachments()
        .object_at(0)
        .unwrap();
    attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);

    // Setup vertex descriptor
    let vertex_descriptor = metal::VertexDescriptor::new();

    // Position attribute
    vertex_descriptor
        .attributes()
        .object_at(0)
        .unwrap()
        .set_format(metal::MTLVertexFormat::Float3);
    vertex_descriptor
        .attributes()
        .object_at(0)
        .unwrap()
        .set_offset(0);
    vertex_descriptor
        .attributes()
        .object_at(0)
        .unwrap()
        .set_buffer_index(0);

    // Color attribute
    vertex_descriptor
        .attributes()
        .object_at(1)
        .unwrap()
        .set_format(metal::MTLVertexFormat::Float4);
    vertex_descriptor
        .attributes()
        .object_at(1)
        .unwrap()
        .set_offset(12); // 3 floats
    vertex_descriptor
        .attributes()
        .object_at(1)
        .unwrap()
        .set_buffer_index(0);

    // Vertex buffer layout
    vertex_descriptor
        .layouts()
        .object_at(0)
        .unwrap()
        .set_stride(28); // 7 floats per vertex (3 for position, 4 for color)

    println!("Vertex descriptor:");
    println!(
        "  Position: format={:?}, offset={}, buffer_index={}",
        vertex_descriptor
            .attributes()
            .object_at(0)
            .unwrap()
            .format(),
        vertex_descriptor
            .attributes()
            .object_at(0)
            .unwrap()
            .offset(),
        vertex_descriptor
            .attributes()
            .object_at(0)
            .unwrap()
            .buffer_index()
    );
    println!(
        "  Color: format={:?}, offset={}, buffer_index={}",
        vertex_descriptor
            .attributes()
            .object_at(1)
            .unwrap()
            .format(),
        vertex_descriptor
            .attributes()
            .object_at(1)
            .unwrap()
            .offset(),
        vertex_descriptor
            .attributes()
            .object_at(1)
            .unwrap()
            .buffer_index()
    );
    println!(
        "  Stride: {}",
        vertex_descriptor.layouts().object_at(0).unwrap().stride()
    );

    pipeline_descriptor.set_vertex_descriptor(Some(vertex_descriptor));

    // Create the render pipeline state
    println!("Render pipeline state created");
    Ok(pipeline_descriptor)
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
