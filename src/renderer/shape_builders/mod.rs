//! This module provides various builder structures and traits for creating
//! and manipulating geometric shapes within the rendering system. It includes
//! implementations for general shape builder as well as specific shapes like triangles.
//!
//! Key components:
//! - `shape_builder`: Provides the core shape building functionality and traits.
//! - `triangle_builder`: Implements a specific builder for triangle shapes.
//! - `MeshBuilder`: A builder for creating mesh objects.
//! - `TriangleBuilder`: A specialized builder for creating triangle primitives.

pub mod shape_builder;
pub mod triangle_builder;

pub use shape_builder::MeshBuilder;
pub use triangle_builder::TriangleBuilder;
