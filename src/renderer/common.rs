//! Common types and structures for the renderer.
//!
//! This module provides various common types, enums, and structures used
//! throughout the renderer, including color representations, vertex definitions,
//! and error type.

use core::fmt;
use glam::Mat4;
use log::error;
use metal::{MTLIndexType, MTLPrimitiveType};
use raw_window_handle::HandleError;
use std::num::NonZeroU32;

/// Represents a texture ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(pub NonZeroU32);

/// Represents different primitive types for rendering.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveType {
    Point,
    Line,
    LineStrip,
    Triangle,
    TriangleStrip,
}

impl From<PrimitiveType> for MTLPrimitiveType {
    fn from(pt: PrimitiveType) -> Self {
        match pt {
            PrimitiveType::Point => MTLPrimitiveType::Point,
            PrimitiveType::Line => MTLPrimitiveType::Line,
            PrimitiveType::LineStrip => MTLPrimitiveType::LineStrip,
            PrimitiveType::Triangle => MTLPrimitiveType::Triangle,
            PrimitiveType::TriangleStrip => MTLPrimitiveType::TriangleStrip,
        }
    }
}

/// Represents different index types for rendering.
#[derive(Debug)]
pub enum IndexType {
    UInt16,
    UInt32,
}

impl From<IndexType> for MTLIndexType {
    fn from(it: IndexType) -> Self {
        match it {
            IndexType::UInt16 => MTLIndexType::UInt16,
            IndexType::UInt32 => MTLIndexType::UInt32,
        }
    }
}

/// Represents a draw command for the backend.
pub enum BackendDrawCommand {
    Basic {
        primitive_type: PrimitiveType,
        vertex_start: u64,
        vertex_count: u64,
    },
    Indexed {
        primitive_type: PrimitiveType,
        index_count: u64,
        index_type: IndexType,
        index_buffer_offset: u64,
    },
    Instanced {
        primitive_type: PrimitiveType,
        vertex_start: u64,
        vertex_count: u64,
        instance_count: u64,
    },
    IndexedInstanced {
        primitive_type: PrimitiveType,
        index_count: u64,
        index_type: IndexType,
        index_buffer_offset: u64,
        instance_count: u64,
    },
}

/// Represents a color with red, green, blue, and alpha components.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Creates a new Color instance.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

/// Represents a vertex with position and color.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

/// Represents uniform data for rendering.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Uniforms {
    pub view_projection_matrix: Mat4,
    pub model_matrix: Mat4,
}

/// Represents possible errors that can occur in the renderer.
#[derive(Debug)]
pub enum RendererError {
    DeviceNotFound,
    ShaderCompilationFailed(String),
    ShaderFunctionNotFound(String),
    PipelineCreationFailed(String),
    DrawFailed(String),
    WindowCreationFailed(String),
    EventLoopError(String),
    WindowHandleError(String),
    BufferOverflow,
    InvalidTextureId,
    InvalidPipelineId,
    InvalidMeshId,
    UnsupportedPlatform,
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RendererError::DeviceNotFound => write!(f, "Metal device not found"),
            RendererError::ShaderCompilationFailed(msg) => {
                write!(f, "Shader compilation failed: {msg}")
            }
            RendererError::ShaderFunctionNotFound(name) => {
                write!(f, "Shader function not found: {name}")
            }
            RendererError::PipelineCreationFailed(msg) => {
                write!(f, "Pipeline creation failed: {msg}")
            }
            RendererError::DrawFailed(msg) => write!(f, "Draw operation failed: {msg}"),
            RendererError::WindowCreationFailed(msg) => {
                write!(f, "Window creation with winit failed: {msg}")
            }
            RendererError::EventLoopError(msg) => {
                write!(f, "Winit event loop error: {msg}")
            }
            RendererError::WindowHandleError(msg) => {
                write!(f, "Winit window handle error: {msg}")
            }
            RendererError::BufferOverflow => {
                write!(f, "Buffer overflow")
            }
            RendererError::InvalidTextureId => {
                write!(f, "Invalid texture Id")
            }
            RendererError::InvalidPipelineId => {
                write!(f, "Invalid pipeline Id")
            }
            RendererError::InvalidMeshId => {
                write!(f, "Invalid mesh Id")
            }
            RendererError::UnsupportedPlatform => {
                write!(f, "Unsupported platform")
            }
        }
    }
}

impl std::error::Error for RendererError {}

impl From<HandleError> for RendererError {
    fn from(error: HandleError) -> Self {
        error!("Window handle error: {}", error);
        RendererError::WindowHandleError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use metal::{MTLIndexType, MTLPrimitiveType};

    use crate::renderer::common::{IndexType, PrimitiveType};

    use super::{Color, Vertex};

    #[test]
    fn test_color_creation() {
        let color = Color::new(0.1, 0.2, 0.3, 0.4);
        assert_eq!(color.r, 0.1);
        assert_eq!(color.g, 0.2);
        assert_eq!(color.b, 0.3);
        assert_eq!(color.a, 0.4);
    }

    #[test]
    fn test_color_to_array() {
        let color = Color::new(0.1, 0.2, 0.3, 0.4);
        let array: [f32; 4] = color.into();
        assert_eq!(array, [0.1, 0.2, 0.3, 0.4]);
    }

    #[test]
    fn test_vertex_default() {
        let vertex = Vertex::default();
        assert_eq!(vertex.position, [0.0, 0.0, 0.0]);
        assert_eq!(vertex.color, [1.0, 1.0, 1.0, 1.0]);
    }

    #[test]
    fn test_primitive_type_conversion() {
        assert_eq!(
            MTLPrimitiveType::from(PrimitiveType::Point),
            MTLPrimitiveType::Point
        );
        assert_eq!(
            MTLPrimitiveType::from(PrimitiveType::Line),
            MTLPrimitiveType::Line
        );
        assert_eq!(
            MTLPrimitiveType::from(PrimitiveType::Triangle),
            MTLPrimitiveType::Triangle
        );
    }

    #[test]
    fn test_index_type_conversion() {
        assert_eq!(MTLIndexType::from(IndexType::UInt16), MTLIndexType::UInt16);
        assert_eq!(MTLIndexType::from(IndexType::UInt32), MTLIndexType::UInt32);
    }
}
