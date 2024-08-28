use super::{
    shape_builder::{vec3_color_to_vertex, PrimitiveBuilder, ShapeBuilder},
    MeshBuilder,
};
use crate::renderer::{
    common::{PrimitiveType, Vertex},
    Color,
};
use glam::Vec3;

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
        MeshBuilder::new()
            .with_vertices(self.vertices.to_vec())
            .with_primitive_type(PrimitiveType::Triangle)
    }
}
