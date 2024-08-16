use core::fmt;

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

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

#[derive(Debug)]
pub enum RendererError {
    DeviceNotFound,
    ShaderCompilationFailed(String),
    ShaderFunctionNotFound(String),
    PipelineCreationFailed(String),
    DrawFailed(String),
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
        }
    }
}

impl std::error::Error for RendererError {}
