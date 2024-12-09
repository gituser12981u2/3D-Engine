//! Renderer Module
//!
//! This module provides a comprehensive rendering system for 3D graphics applications.
//! It includes components for managing the rendering backend, camera, meshes, and various
//! utility structures for efficient rendering.
//!
//! Key Components:
//!
//! - `backend`: Handles the low-level graphics API interactions (e.g., Metal, Vulkan).
//! - `camera`: Provides a camera system for 3D scene navigation and projection.
//! - `common`: Contains common data structures and types used throughout the renderer.
//! - `render_core`: Implements the core rendering logic and system management.
//! - `render_queue`: Handles the queuing and processing of draw commands.
//! - `shape_builders`: Offers utilities for creating various 3D shapes programmatically.
//!
//! This module abstracts away much of the complexity of 3D rendering, providing a
//! high-level interface for creating and managing 3D scenes while maintaining
//! flexibility for advanced usage.

mod backend;
mod camera;
mod common;
mod mesh;
mod render_core;
mod render_queue;
pub mod shape_builders;

pub use self::common::{Color, RendererError};
pub use camera::Camera;
pub use render_core::RendererSystem;
pub use render_queue::{DrawCommandBuilder, InstanceData};
