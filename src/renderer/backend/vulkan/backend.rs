use crate::renderer::backend::GraphicsBackend;

pub struct VulkanBackend {
    // TODO: Add Vulkan-specific fields
}

impl VulkanBackend {
    #[allow(dead_code)]
    pub fn new() -> Self {
        // TODO: Initialize Vulkan
        unimplemented!()
    }
}

impl GraphicsBackend for VulkanBackend {
    #[allow(unused_variables)]
    fn draw(
        &mut self,
        draw_command: crate::renderer::common::BackendDrawCommand,
    ) -> Result<(), crate::renderer::RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_vertex_buffer(
        &mut self,
        vertices: &[crate::renderer::common::Vertex],
    ) -> Result<(), crate::renderer::RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_index_buffer(
        &mut self,
        indices: &[u32],
    ) -> Result<(), crate::renderer::RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_uniform_buffer(
        &mut self,
        uniforms: &crate::renderer::common::Uniforms,
    ) -> Result<(), crate::renderer::RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_instance_buffer(
        &mut self,
        instances: &[crate::renderer::render_queue::InstanceData],
    ) -> Result<(), crate::renderer::RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn create_texture(
        &mut self,
        descriptor: &metal::TextureDescriptor,
    ) -> crate::renderer::common::TextureId {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_texture(
        &mut self,
        id: crate::renderer::common::TextureId,
        region: metal::MTLRegion,
        mipmap_level: u64,
        slice: u64,
        bytes: &[u8],
        bytes_per_row: u64,
        bytes_per_image: u64,
    ) -> Result<(), crate::renderer::RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn create_render_pipeline_state(
        &mut self,
        descriptor: &metal::RenderPipelineDescriptor,
    ) -> Result<(), crate::renderer::RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn render_pass(
        &mut self,
        descriptor: &metal::RenderPassDescriptorRef,
    ) -> Result<crate::renderer::backend::metal::RenderPass, crate::renderer::RendererError> {
        unimplemented!()
    }
}
