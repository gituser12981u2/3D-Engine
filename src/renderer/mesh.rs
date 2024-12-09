//! Mesh and mesh storage module for the renderer.
//!
//! This module provides structures and implementations for creating and managing
//! meshes, as well as storing them efficiently for use in rendering.

use super::{
    common::{PrimitiveType, Vertex},
    shape_builders::MeshBuilder,
};
use crate::debug_trace;
use log::{debug, trace};

/// Represents a mesh with vertices, indices, and associated Metal buffers.
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
    pub primitive_type: PrimitiveType,
}

impl Mesh {
    /// Create a new Mesh from a `MeshBuilder`.
    ///
    /// # Arguments
    ///
    /// * `mesh_builder` - The `MeshBuilder` containing mesh data.
    ///
    /// # Returns
    ///
    /// A new Mesh instance.
    pub fn new(mesh_builder: MeshBuilder) -> Self {
        debug_trace!("Creating new Mesh");
        Mesh {
            vertices: mesh_builder.data.vertices,
            indices: mesh_builder.data.indices,
            primitive_type: mesh_builder.data.primitive_type,
        }
    }
}

/// Stores and manages multiple Mesh instances.
pub struct MeshStorage {
    meshes: Vec<Mesh>,
}

impl MeshStorage {
    /// Creates a new `MeshStorage` instance.
    ///
    /// # Returns
    ///
    /// A new `MeshStorage` instance.
    pub fn new() -> Self {
        debug!("Creating new MeshStorage");
        Self { meshes: Vec::new() }
    }

    /// Adds a new mesh to the storage.
    ///
    /// # Arguments
    ///
    /// * `mesh_builder` - The `MeshBuilder` to create the mesh from.
    ///
    /// # Returns
    ///
    /// The index of the newly added mesh.
    pub fn add_mesh(&mut self, mesh_builder: MeshBuilder) -> usize {
        let mesh = Mesh::new(mesh_builder);
        self.meshes.push(mesh);
        let index = self.meshes.len() - 1;
        debug_trace!("Added new mesh to MeshStorage at index {}", index);
        index
    }

    /// Retrieves a reference to a mesh by its index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the mesh to retrieve.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the Mesh if found, or None if not found.
    pub fn get_mesh(&self, index: usize) -> Option<&Mesh> {
        let mesh = self.meshes.get(index);
        if mesh.is_some() {
            trace!("Retrieved mesh at index {}", index);
        } else {
            debug!("Failed to retrieve mesh at index {}", index);
        }
        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::{Mesh, MeshStorage};
    use crate::renderer::{
        common::{PrimitiveType, Vertex},
        shape_builders::MeshBuilder,
    };

    fn create_test_mesh_builder() -> MeshBuilder {
        let vertices = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ];
        MeshBuilder::new(vertices, PrimitiveType::Triangle)
    }

    #[test]
    fn test_mesh_creation() {
        let mesh_builder = create_test_mesh_builder();
        let mesh = Mesh::new(mesh_builder);

        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.primitive_type, PrimitiveType::Triangle);
        assert!(mesh.indices.is_none());
    }

    #[test]
    fn test_mesh_storage() {
        let mut storage = MeshStorage::new();

        let mesh_builder = create_test_mesh_builder();
        let mesh_id = storage.add_mesh(mesh_builder);
        assert_eq!(mesh_id, 0);

        let retrieved_mesh = storage.get_mesh(mesh_id);
        assert!(retrieved_mesh.is_some());

        let mesh = retrieved_mesh.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.primitive_type, PrimitiveType::Triangle);
    }
}

use super::{
    common::{PrimitiveType, Vertex},
    shape_builders::MeshBuilder,
};
use log::{debug, trace};

/// Represents a mesh with vertices, indices, and associated Metal buffers.
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
    pub primitive_type: PrimitiveType,
}

impl Mesh {
    /// Create a new Mesh from a MeshBuilder.
    ///
    /// # Arguments
    ///
    /// * `mesh_builder` - The `MeshBuilder` containing mesh data.
    ///
    /// # Returns
    ///
    /// A new Mesh instance.
    pub fn new(mesh_builder: MeshBuilder) -> Self {
        debug_trace!("Creating new Mesh");
        Mesh {
            vertices: mesh_builder.data.vertices,
            indices: mesh_builder.data.indices,
            primitive_type: mesh_builder.data.primitive_type,
        }
    }
}

/// Stores and manages multiple Mesh instances.
pub struct MeshStorage {
    meshes: Vec<Mesh>,
}

impl MeshStorage {
    /// Creates a new `MeshStorage` instance.
    ///
    /// # Returns
    ///
    /// A new `MeshStorage` instance.
    pub fn new() -> Self {
        debug!("Creating new MeshStorage");
        Self { meshes: Vec::new() }
    }

    /// Adds a new mesh to the storage.
    ///
    /// # Arguments
    ///
    /// * `mesh_builder` - The `MeshBuilder` to create the mesh from.
    ///
    /// # Returns
    ///
    /// The index of the newly added mesh.
    pub fn add_mesh(&mut self, mesh_builder: MeshBuilder) -> usize {
        let mesh = Mesh::new(mesh_builder);
        self.meshes.push(mesh);
        let index = self.meshes.len() - 1;
        debug_trace!("Added new mesh to MeshStorage at index {}", index);
        index
    }

    /// Retrieves a reference to a mesh by its index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the mesh to retrieve.
    ///
    /// # Returns
    ///
    /// An option containing a reference to the Mesh if found, or None if not found.
    pub fn get_mesh(&self, index: usize) -> Option<&Mesh> {
        let mesh = self.meshes.get(index);
        if mesh.is_some() {
            trace!("Retrieved mesh at index {}", index);
        } else {
            debug!("Failed to retrieve mesh at index {}", index);
        }
        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::{Mesh, MeshStorage};
    use crate::renderer::{
        common::{PrimitiveType, Vertex},
        shape_builders::MeshBuilder,
    };

    fn create_test_mesh_builder() -> MeshBuilder {
        let vertices = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ];
        MeshBuilder::new(vertices, PrimitiveType::Triangle)
    }

    #[test]
    fn test_mesh_creation() {
        let mesh_builder = create_test_mesh_builder();
        let mesh = Mesh::new(mesh_builder);

        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.primitive_type, PrimitiveType::Triangle);
        assert!(mesh.indices.is_none());
    }

    #[test]
    fn test_mesh_storage() {
        let mut storage = MeshStorage::new();

        let mesh_builder = create_test_mesh_builder();
        let mesh_id = storage.add_mesh(mesh_builder);
        assert_eq!(mesh_id, 0);

        let retrieved_mesh = storage.get_mesh(mesh_id);
        assert!(retrieved_mesh.is_some());

        let mesh = retrieved_mesh.unwrap();
        assert_eq!(mesh.vertices.len(), 3);
        assert_eq!(mesh.primitive_type, PrimitiveType::Triangle);
    }
}
