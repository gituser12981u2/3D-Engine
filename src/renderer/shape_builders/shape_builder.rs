//! Shape builder module for the renderer.
//!
//! This module provides structures and traits for creating and manipulating
//! primitive shapes and meshes. It includes the `ShapeBuilder` trait, which
//! allows conversion between different shape representations, and the
//! `PrimitiveBuilder` and `MeshBuilder` structs for detailed shape customization.

use crate::renderer::{
    common::{PrimitiveType, Vertex},
    render_core::Renderer,
    Color, DrawCommandBuilder, InstanceData,
};
use glam::{Mat4, Vec3};

/// Trait for converting shapes into primitive or mesh builders.
#[allow(clippy::wrong_self_convention)]
// as_* is a better naming scheme for API
#[allow(dead_code)]
pub trait ShapeBuilder {
    fn as_primitive(self) -> PrimitiveBuilder;
    fn as_mesh(self) -> MeshBuilder;
}

/// Builder for creating and customizing shapes.
#[derive(Clone)]
pub struct ShapeData {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
    pub primitive_type: PrimitiveType,
    pub transform: Mat4,
    pub instances: Option<Vec<InstanceData>>,
}

impl ShapeData {
    /// Creates a new `ShapeData` with the given vertices and primitive type.
    pub fn new(vertices: Vec<Vertex>, primitive_type: PrimitiveType) -> Self {
        Self {
            vertices,
            indices: None,
            primitive_type,
            transform: Mat4::IDENTITY,
            instances: None,
        }
    }

    /// Adds indices to the primitive.
    fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = Some(indices);
        self
    }

    /// Applies a transformation to the primitive.
    fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self
    }

    /// Adds instances to the primitive.
    fn with_instances(mut self, instances: Vec<InstanceData>) -> Self {
        self.instances = Some(instances);
        self
    }
}

impl ShapeBuilder for ShapeData {
    fn as_primitive(self) -> PrimitiveBuilder {
        PrimitiveBuilder { data: self }
    }

    fn as_mesh(self) -> MeshBuilder {
        MeshBuilder { data: self }
    }
}

/// Builder for creating and customizing primitive shapes.
pub struct PrimitiveBuilder {
    data: ShapeData,
}

impl PrimitiveBuilder {
    /// Creates a new `PrimitiveBuilder` with the given vertices and primitive type.
    pub fn new(vertices: Vec<Vertex>, primitive_type: PrimitiveType) -> Self {
        Self {
            data: ShapeData::new(vertices, primitive_type),
        }
    }

    /// Adds indices to the primitive.
    #[allow(dead_code)]
    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.data = self.data.with_indices(indices);
        self
    }

    /// Applies a transformation to the primitive.
    #[allow(dead_code)]
    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.data = self.data.with_transform(transform);
        self
    }

    /// Adds instances to the primitive.
    #[allow(dead_code)]
    pub fn with_instances(mut self, instances: Vec<InstanceData>) -> Self {
        self.data = self.data.with_instances(instances);
        self
    }

    /// Draws the primitive using the provided renderer.
    #[allow(dead_code)]
    pub fn draw(self, renderer: &mut Renderer) {
        let mut draw_command = DrawCommandBuilder::new_primitive(
            self.data.vertices,
            self.data.indices,
            self.data.primitive_type,
        )
        .with_transform(self.data.transform);

        if let Some(instances) = self.data.instances {
            draw_command = draw_command.with_instances(instances);
        }

        renderer.draw_immediate(draw_command.build());
    }
}

/// Builder for creating and customizing mesh shapes.
#[derive(Clone)]
pub struct MeshBuilder {
    pub data: ShapeData,
}

impl MeshBuilder {
    /// Creates a new `MeshBuilder` with default values.
    pub fn new(vertices: Vec<Vertex>, primitive_type: PrimitiveType) -> Self {
        Self {
            data: ShapeData::new(vertices, primitive_type),
        }
    }

    /// Adds indices to the primitive.
    #[allow(dead_code)]
    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.data = self.data.with_indices(indices);
        self
    }

    /// Applies a transformation to the primitive.
    #[allow(dead_code)]
    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.data = self.data.with_transform(transform);
        self
    }

    /// Adds instances to the primitive.
    #[allow(dead_code)]
    pub fn with_instances(mut self, instances: Vec<InstanceData>) -> Self {
        self.data = self.data.with_instances(instances);
        self
    }

    /// Draws the mesh using the provided renderer.
    #[allow(dead_code)]
    pub fn draw(&self, renderer: &mut Renderer) {
        let mesh_id = renderer.add_mesh(self.clone());
        let mut draw_command =
            DrawCommandBuilder::new_mesh(mesh_id).with_transform(self.data.transform);

        if let Some(instances) = &self.data.instances {
            draw_command = draw_command.with_instances(instances.clone());
        }

        renderer.draw_immediate(draw_command.build());
    }
}

// !! Delete and make it Vertex only for better performance
/// Converts a Vec3 position and Color to  a Vertex.
pub fn vec3_color_to_vertex(position: Vec3, color: Color) -> Vertex {
    Vertex {
        position: position.to_array(),
        color: color.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{vec3_color_to_vertex, MeshBuilder, PrimitiveBuilder};
    use crate::renderer::{
        common::{PrimitiveType, Vertex},
        Color, InstanceData,
    };
    use glam::{Mat4, Vec3};

    // Helper function to create a sample triangle
    fn create_sample_triangle() -> Vec<Vertex> {
        vec![
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
        ]
    }

    #[test]
    fn test_primitive_builder() {
        let vertices = create_sample_triangle();
        let builder = PrimitiveBuilder::new(vertices.clone(), PrimitiveType::Triangle)
            .with_indices(vec![0, 1, 2])
            .with_transform(Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)))
            .with_instances(vec![InstanceData::new(
                Mat4::IDENTITY,
                Color::new(1.0, 0.0, 0.0, 1.0),
            )]);

        assert_eq!(builder.data.vertices, vertices);
        assert_eq!(builder.data.primitive_type, PrimitiveType::Triangle);
        assert_eq!(builder.data.indices, Some(vec![0, 1, 2]));
        assert_eq!(
            builder.data.transform,
            Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0))
        );
        assert!(builder.data.instances.is_some());
    }

    #[test]
    fn test_mesh_builder() {
        let vertices = create_sample_triangle();
        let builder = MeshBuilder::new(vertices.clone(), PrimitiveType::Triangle)
            .with_indices(vec![0, 1, 2])
            .with_transform(Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)))
            .with_instances(vec![InstanceData::new(
                Mat4::IDENTITY,
                Color::new(1.0, 0.0, 0.0, 1.0),
            )]);

        assert_eq!(builder.data.vertices, vertices);
        assert_eq!(builder.data.primitive_type, PrimitiveType::Triangle);
        assert_eq!(builder.data.indices, Some(vec![0, 1, 2]));
        assert_eq!(
            builder.data.transform,
            Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0))
        );
        assert!(builder.data.instances.is_some());
    }

    #[test]
    fn test_vec3_color_to_vertex() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        let color = Color::new(0.1, 0.2, 0.3, 1.0);
        let vertex = vec3_color_to_vertex(position, color);
        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.color, [0.1, 0.2, 0.3, 1.0]);
    }
}
