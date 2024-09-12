//! Graphics backend module for the renderer.
//!
//! This module defines the `GraphicsBackend` trait, which provides an interface
//! for different graphics APIs (such as Metal and Vulkan) to implement. It also
//! re-exports the specific backend implementations.
//!
//! The `GraphicsBackend` trait defines methods for:
//! - Rendering operations
//! - Buffer management (vertex, index, uniform, and instance buffers)
//! - Texture creation and updates
//! - Render pipeline state creation
//!
//! Implementations of this trait allow the renderer to work with different
//! graphics APIs in a unified manner.

pub mod metal;
pub mod vulkan;

use super::{
    common::{BackendDrawCommand, RendererError, TextureId, Uniforms, Vertex},
    render_queue::InstanceData,
};
use ::metal::{MTLRegion, RenderPassDescriptorRef, RenderPipelineDescriptor, TextureDescriptor};

/// Trait defining the interface for graphics backends.
///
/// Implementations of this trait provide the necessary methods for rendering,
/// buffer management, and pipeline state creation for specific graphics APIs
/// and allows for proper abstraction of the backend and renderer.
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
