pub mod metal;
pub mod vulkan;

use super::{
    common::{BackendDrawCommand, RendererError, TextureId, Uniforms, Vertex},
    render_queue::InstanceData,
};
use ::metal::{MTLRegion, RenderPassDescriptorRef, RenderPipelineDescriptor, TextureDescriptor};

pub trait GraphicsBackend {
    #[allow(dead_code)]
    fn render_pass(&mut self, descriptor: &RenderPassDescriptorRef) -> Result<(), RendererError>;
    fn draw(&mut self, draw_command: BackendDrawCommand) -> Result<(), RendererError>;

    fn update_vertex_buffer(&mut self, vertices: &[Vertex]) -> Result<(), RendererError>;
    fn update_index_buffer(&mut self, indices: &[u32]) -> Result<(), RendererError>;
    fn update_uniform_buffer(&mut self, uniforms: &Uniforms) -> Result<(), RendererError>;
    fn update_instance_buffer(&mut self, instances: &[InstanceData]) -> Result<(), RendererError>;

    #[allow(dead_code)]
    fn create_texture(&mut self, descriptor: &TextureDescriptor) -> TextureId;

    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    fn update_texture(
        &mut self,
        id: TextureId,
        region: MTLRegion,
        mipmap_level: u64,
        slice: u64,
        bytes: &[u8],
        bytes_per_row: u64,
        bytes_per_image: u64,
    ) -> Result<(), RendererError>;

    #[allow(dead_code)]
    fn create_render_pipeline_state(
        &mut self,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<(), RendererError>;
}
