//! A Metal-based 3D rendering library with a focus on performance
//!
//! This library provides a comprehensive rendering system including camera controls,
//! primitive shape construction, mesh management, and an efficient rendering pipeline.

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_trace {
    ($($arg:tt)*) => ( log::trace!($($arg)*) );
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_trace {
    ($($arg:tt)*) => {};
}

// Core components
pub use crate::renderer::{
    // Basic types
    Camera,
    Color,
    // Error handling
    RendererError,
    // Core renderer system
    RendererSystem,
};

// Shape builders
pub use crate::renderer::shape_builders::{
    shape_builder::{vec3_color_to_vertex, MeshBuilder, PrimitiveBuilder, ShapeBuilder, ShapeData},
    triangle_builder::TriangleBuilder,
};

pub mod renderer;
