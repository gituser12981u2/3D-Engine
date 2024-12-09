//! Metal backend implementation for the renderer.
//!
//! This module contains the components necessary for rendering the graphics using the Metal API.
//! It includes implementations for backend operations, buffer management, pipeline creation,
//! and texture management specific to Metal.
//!
//! Key components:
//! - `backend`: Implements the core Metal backend functionality.
//! - `buffer_management`: Handles creation and management of Metal buffers.
//! - `pipeline`: Manages creation and caching of render pipeline states.
//! - `texture_manager`: Handles creation and management of Metal textures.

mod backend;
mod buffer_manager;
mod pipeline;
mod texture_manager;

pub use self::backend::MetalBackend;
