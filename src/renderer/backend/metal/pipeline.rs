use crate::renderer::RendererError;
use metal::{Device, MTLPixelFormat, RenderPipelineState};

pub fn create_render_pipeline(device: &Device) -> Result<RenderPipelineState, RendererError> {
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

    let vertex_function = library.get_function("vertex_main", None).map_err(|e| {
        println!("Failed to get vertex_main function: {:?}", e);
        RendererError::ShaderFunctionNotFound("vertex_main".to_string())
    })?;
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

    println!("Render pipeline state created");
    // Create the render pipeline state
    device
        .new_render_pipeline_state(&pipeline_descriptor)
        .map_err(|e| RendererError::PipelineCreationFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::renderer::backend::metal::pipeline::create_render_pipeline;
    use metal::Device;

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
