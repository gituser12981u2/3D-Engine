use crate::renderer::{
    common::{PrimitiveType, Vertex},
    render_core::Renderer,
    Color, DrawCommandBuilder, InstanceData,
};
use glam::{Mat4, Vec3};

#[allow(clippy::wrong_self_convention)]
#[allow(dead_code)]
// as_* is a better naming scheme for API
pub trait ShapeBuilder {
    fn as_primitive(self) -> PrimitiveBuilder;
    fn as_mesh(self) -> MeshBuilder;
}

/// Allows creation and customization of primitive shapes.
///
/// # Example
///
/// ```
/// renderer.create_triangle(
///     Vec3::new(0.0, 0.5, 0.0),
///     Vec3::new(-0.5, -0.5, 0.0),
///     Vec3::new(0.5, -0.5, 0.0),
///     Color::new(1.0, 0.0, 0.0, 1.0)
/// )
/// .as_primitive()
/// .with_indices(vec![0, 1, 2])
/// .with_transform(Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0)))
/// .draw(renderer);
/// ```
#[allow(dead_code)]
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

    /// Adds indices to the primitive.
    ///
    /// # Example
    ///
    /// ```
    /// .with_indices(vec![0, 1, 2])
    /// ```
    ///
    #[allow(dead_code)]
    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = Some(indices);
        self
    }

    /// Applies a transformation to the primitive.
    ///
    /// # Example
    ///
    /// ```
    /// .with_transform(Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0)))
    /// ```
    #[allow(dead_code)]
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
    pub fn as_primitive(self) -> PrimitiveBuilder {
        self
    }

    #[allow(dead_code)]
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

/// Allows creation and customization of mesh shapes.
///
/// # Example
///
/// ```
/// renderer.create_triangle(
///     Vec3::new(0.0, 0.5, 0.0),
///     Vec3::new(-0.5, -0.5, 0.0),
///     Vec3::new(0.5, -0.5, 0.0),
///     Color::new(1.0, 0.0, 0.0, 1.0)
/// )
/// .as_mesh()
/// .with_indices(vec![0, 1, 2])
/// .with_transform(Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0)))
/// .draw(renderer);
/// ```
pub struct MeshBuilder {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
    pub primitive_type: PrimitiveType,
    pub transform: Mat4,
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

    pub fn with_primitive_type(mut self, primitive_type: PrimitiveType) -> Self {
        self.primitive_type = primitive_type;
        self
    }

    /// Adds indices to the mesh.
    ///
    /// # Example
    ///
    /// ```
    /// .with_indices(vec![0, 1, 2])
    /// ```
    #[allow(dead_code)]
    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = Some(indices);
        self
    }

    /// Applies a transformation to the mesh.
    ///
    /// # Example
    ///
    /// ```
    /// .with_transform(Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0)))
    /// ```
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::{vec3_color_to_vertex, vertex_to_vec3_color, MeshBuilder, PrimitiveBuilder};
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
    fn test_primitive_builder_creation() {
        let vertices = create_sample_triangle();
        let builder = PrimitiveBuilder::new(vertices.clone(), PrimitiveType::Triangle);
        assert_eq!(builder.vertices, vertices);
        assert_eq!(builder.primitive_type, PrimitiveType::Triangle);
    }

    #[test]
    fn test_primitive_builder_with_indices() {
        let vertices = create_sample_triangle();
        let indices = vec![0, 1, 2];
        let builder =
            PrimitiveBuilder::new(vertices, PrimitiveType::Triangle).with_indices(indices.clone());
        assert_eq!(builder.indices, Some(indices));
    }

    #[test]
    fn test_primitive_builder_with_transform() {
        let vertices = create_sample_triangle();
        let transform = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        let builder =
            PrimitiveBuilder::new(vertices, PrimitiveType::Triangle).with_transform(transform);
        assert_eq!(builder.transform, transform);
    }

    #[test]
    fn test_primitive_builder_with_instances() {
        let vertices = create_sample_triangle();
        let instances = vec![
            InstanceData::new(Mat4::IDENTITY, Color::new(1.0, 0.0, 0.0, 1.0)),
            InstanceData::new(
                Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)),
                Color::new(0.0, 1.0, 0.0, 1.0),
            ),
        ];
        let builder = PrimitiveBuilder::new(vertices, PrimitiveType::Triangle)
            .with_instances(instances.clone());
        assert_eq!(builder.instances, Some(instances));
    }

    #[test]
    fn test_primitive_builder_as_primitive() {
        let vertices = create_sample_triangle();
        let builder = PrimitiveBuilder::new(vertices.clone(), PrimitiveType::Triangle);
        let primitive_builder = builder.as_primitive();
        assert_eq!(primitive_builder.vertices, vertices);
    }

    #[test]
    fn test_mesh_builder_creation() {
        let builder = MeshBuilder::new();
        assert!(builder.vertices.is_empty());
        assert_eq!(builder.primitive_type, PrimitiveType::Triangle)
    }

    #[test]
    fn test_mesh_builder_with_vertices() {
        let vertices = create_sample_triangle();
        let builder = MeshBuilder::new().with_vertices(vertices.clone());
        assert_eq!(builder.vertices, vertices);
    }

    #[test]
    fn test_mesh_builder_with_indices() {
        let indices = vec![0, 1, 2];
        let builder = MeshBuilder::new().with_indices(indices.clone());
        assert_eq!(builder.indices, Some(indices));
    }

    #[test]
    fn test_mesh_builder_with_primitive_type() {
        let builder = MeshBuilder::new().with_primitive_type(PrimitiveType::Line);
        assert_eq!(builder.primitive_type, PrimitiveType::Line);
    }

    #[test]
    fn test_mesh_builder_with_instances() {
        let instances = vec![
            InstanceData::new(Mat4::IDENTITY, Color::new(1.0, 0.0, 0.0, 1.0)),
            InstanceData::new(
                Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0)),
                Color::new(0.0, 1.0, 0.0, 1.0),
            ),
        ];
        let builder = MeshBuilder::new().with_instances(instances.clone());
        assert_eq!(builder.instances, Some(instances));
    }

    #[test]
    fn test_mesh_builder_as_mesh() {
        let vertices = create_sample_triangle();
        let builder = MeshBuilder::new().with_vertices(vertices.clone());
        let mesh_builder = builder.as_mesh();
        assert_eq!(mesh_builder.vertices, vertices);
    }

    #[test]
    fn test_vec3_color_to_vertex() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        let color = Color::new(0.1, 0.2, 0.3, 1.0);
        let vertex = vec3_color_to_vertex(position, color);
        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.color, [0.1, 0.2, 0.3, 1.0]);
    }

    #[test]
    fn test_vertex_to_vec3_color() {
        let vertex = Vertex {
            position: [1.0, 2.0, 3.0],
            color: [0.1, 0.2, 0.3, 1.0],
        };
        let (position, color) = vertex_to_vec3_color(vertex);
        assert_eq!(position, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(color, Color::new(0.1, 0.2, 0.3, 1.0));
    }
}
