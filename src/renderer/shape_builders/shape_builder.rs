use glam::{Mat4, Vec3};

use crate::renderer::{
    common::{PrimitiveType, Vertex},
    render_core::Renderer,
    Color, DrawCommandBuilder, InstanceData,
};

#[allow(clippy::wrong_self_convention)]
// as_* is a better naming scheme for API
pub trait ShapeBuilder {
    fn as_primitive(self) -> PrimitiveBuilder;
    fn as_mesh(self) -> MeshBuilder;
}

pub struct PrimitiveBuilder {
    vertices: Vec<Vertex>,
    indices: Option<Vec<u32>>,
    primitive_type: PrimitiveType,
    transform: Mat4,
    instances: Option<Vec<InstanceData>>,
}

impl PrimitiveBuilder {
    pub fn new(vertices: Vec<Vertex>, primitive_type: PrimitiveType) -> Self {
        PrimitiveBuilder {
            vertices,
            indices: None,
            primitive_type,
            transform: Mat4::IDENTITY,
            instances: None,
        }
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = Some(indices);
        self
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_instances(mut self, instances: Vec<InstanceData>) -> Self {
        self.instances = Some(instances);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn as_primitive(self) -> PrimitiveBuilder {
        self
    }

    pub fn draw(self, renderer: &mut Renderer) {
        let mut draw_command =
            DrawCommandBuilder::new_primitive(self.vertices, self.indices, self.primitive_type)
                .with_transform(self.transform);

        if let Some(instances) = self.instances {
            draw_command = draw_command.with_instances(instances)
        }

        renderer.draw_immediate(draw_command.build());
    }
}

pub struct MeshBuilder {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
    pub primitive_type: PrimitiveType,
    transform: Mat4,
    instances: Option<Vec<InstanceData>>,
}

impl MeshBuilder {
    pub fn new() -> Self {
        MeshBuilder {
            vertices: Vec::new(),
            indices: None,
            primitive_type: PrimitiveType::Triangle,
            transform: Mat4::IDENTITY,
            instances: None,
        }
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex>) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = Some(indices);
        self
    }

    pub fn with_primitive_type(mut self, primitive_type: PrimitiveType) -> Self {
        self.primitive_type = primitive_type;
        self
    }

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        self.transform = transform;
        self
    }

    #[allow(dead_code)]
    pub fn with_instances(mut self, instances: Vec<InstanceData>) -> Self {
        self.instances = Some(instances);
        self
    }

    #[allow(clippy::wrong_self_convention)]
    #[allow(dead_code)]
    pub fn as_mesh(self) -> MeshBuilder {
        self
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        let mesh_id = renderer.add_mesh(self.clone());
        let mut draw_command = DrawCommandBuilder::new_mesh(mesh_id).with_transform(self.transform);

        if let Some(instances) = &self.instances {
            draw_command = draw_command.with_instances(instances.clone());
        }

        renderer.draw_immediate(draw_command.build());
    }
}

impl Clone for MeshBuilder {
    fn clone(&self) -> Self {
        MeshBuilder {
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
            primitive_type: self.primitive_type,
            transform: self.transform,
            instances: self.instances.clone(),
        }
    }
}

// !! Probably delete and make it Vertex only for better performance
// Utility functions for Vec3/Color to Vertex conversion
pub fn vec3_color_to_vertex(position: Vec3, color: Color) -> Vertex {
    Vertex {
        position: position.to_array(),
        color: color.into(),
    }
}

#[allow(dead_code)]
pub fn vertex_to_vec3_color(vertex: Vertex) -> (Vec3, Color) {
    (
        Vec3::from_array(vertex.position),
        Color::new(
            vertex.color[0],
            vertex.color[1],
            vertex.color[2],
            vertex.color[3],
        ),
    )
}
