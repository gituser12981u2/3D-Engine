use core::fmt;

use raw_window_handle::HandleError;

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
    UnsupportedPlatform,
    WindowHandleError(String),
    BufferOverflow,
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
        }
    }
}

impl std::error::Error for RendererError {}
