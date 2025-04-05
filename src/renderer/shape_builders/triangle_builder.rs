//! Triangle building module for the renderer.
//!
//! This module provides the `TriangleBuilder` struct for creating simple
//! triangle primitives. It demonstrates how to implement the `ShapeBuilder`
//! trait for custom shapes.

use super::{
    shape_builder::{vec3_color_to_vertex, PrimitiveBuilder, ShapeBuilder},
    MeshBuilder,
};
use crate::renderer::{
    common::{PrimitiveType, Vertex},
    Color,
};
use glam::Vec3;

/// Builder for creating a simple triangle primitive
pub struct TriangleBuilder {
    vertices: [Vertex; 3],
}

impl TriangleBuilder {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3, color: Color) -> Self {
        TriangleBuilder {
            vertices: [
                vec3_color_to_vertex(v1, color),
                vec3_color_to_vertex(v2, color),
                vec3_color_to_vertex(v3, color),
            ],
        }
    }
}

impl ShapeBuilder for TriangleBuilder {
    fn as_primitive(self) -> PrimitiveBuilder {
        PrimitiveBuilder::new(self.vertices.to_vec(), PrimitiveType::Triangle)
    }

    fn as_mesh(self) -> MeshBuilder {
        MeshBuilder::new(self.vertices.to_vec(), PrimitiveType::Triangle)
    }
}

#[cfg(test)]
mod tests {
    use super::TriangleBuilder;
    use crate::renderer::Color;
    use glam::Vec3;

    #[test]
    fn test_triangle_builder() {
        let color = Color::new(1.0, 0.0, 0.0, 1.0);
        let triangle = TriangleBuilder::new(
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.5, -0.5, 0.0),
            color,
        );

        assert_eq!(triangle.vertices.len(), 3);
        assert_eq!(triangle.vertices[0].position, [0.0, 0.5, 0.0]);
        assert_eq!(triangle.vertices[1].position, [-0.5, -0.5, 0.0]);
        assert_eq!(triangle.vertices[2].position, [0.5, -0.5, 0.0]);
    }
}
