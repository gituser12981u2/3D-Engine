use crate::renderer::{
    backend::GraphicsBackend,
    common::{BackendDrawCommand, TextureId, Uniforms, Vertex},
    InstanceData, RendererError,
};

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
    fn draw(&mut self, draw_command: BackendDrawCommand) -> Result<(), RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_vertex_buffer(&mut self, vertices: &[Vertex]) -> Result<(), RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_index_buffer(&mut self, indices: &[u32]) -> Result<(), RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_uniform_buffer(&mut self, uniforms: &Uniforms) -> Result<(), RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_instance_buffer(&mut self, instances: &[InstanceData]) -> Result<(), RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn create_texture(&mut self, descriptor: &metal::TextureDescriptor) -> TextureId {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn update_texture(
        &mut self,
        id: TextureId,
        region: metal::MTLRegion,
        mipmap_level: u64,
        slice: u64,
        bytes: &[u8],
        bytes_per_row: u64,
        bytes_per_image: u64,
    ) -> Result<(), RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn create_render_pipeline_state(
        &mut self,
        descriptor: &metal::RenderPipelineDescriptor,
    ) -> Result<(), RendererError> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn render_pass(
        &mut self,
        descriptor: &metal::RenderPassDescriptorRef,
    ) -> Result<(), RendererError> {
        unimplemented!()
    }
}
