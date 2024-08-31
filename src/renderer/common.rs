use core::fmt;
use metal::{MTLIndexType, MTLPrimitiveType};
use raw_window_handle::HandleError;
use std::num::NonZeroU32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BufferId(pub NonZeroU32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct RenderPipelineId(pub NonZeroU32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(pub NonZeroU32);

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

impl From<IndexType> for MTLIndexType {
    fn from(it: IndexType) -> Self {
        match it {
            IndexType::UInt16 => MTLIndexType::UInt16,
            IndexType::UInt32 => MTLIndexType::UInt32,
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveType {
    Point,
    Line,
    LineStrip,
    Triangle,
    TriangleStrip,
}

#[derive(Debug)]
pub enum IndexType {
    UInt16,
    UInt32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        [color.r, color.g, color.b, color.a]
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
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

impl From<HandleError> for RendererError {
    fn from(error: HandleError) -> Self {
        RendererError::WindowHandleError(error.to_string())
    }
}

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
            RendererError::UnsupportedPlatform => {
                write!(f, "Unsupported platform")
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
        }
    }
}

impl std::error::Error for RendererError {}
