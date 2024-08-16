pub mod metal;
pub mod vulkan;

use super::common::{RendererError, Vertex};

pub trait GraphicsBackend {
    fn prepare_frame(&mut self) -> Result<(), RendererError>;
    fn update_vertex_buffer(&mut self, vertices: &[Vertex]) -> Result<(), RendererError>;
    fn update_index_buffer(&mut self, indices: &[u32]) -> Result<(), RendererError>;
    fn draw(&mut self, vertex_count: u32, index_count: u32) -> Result<(), RendererError>;
    fn present_frame(&mut self) -> Result<(), RendererError>;
}
