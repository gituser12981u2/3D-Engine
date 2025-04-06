use glam::{Quat, Vec3};
use std::f32::consts::PI;

use crate::{
    renderer::{render_core::Renderer, scene_objects::SceneObject, Color, RendererError},
    ShapeBuilder,
};

/// Factory methods for creating common 3D shapes as scene objects
pub struct ShapeFactory;

impl ShapeFactory {
    /// Create a cube with the given properties
    pub fn create_cube(
        renderer: &mut Renderer,
        size: f32,
        color: Color,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Result<SceneObject, RendererError> {
        let half_size = size / 2.0;

        // Define the 8 vertices of the cube
        let vertices = vec![
            // Front face
            (Vec3::new(-half_size, -half_size, half_size), color),
            (Vec3::new(half_size, -half_size, half_size), color),
            (Vec3::new(half_size, half_size, half_size), color),
            (Vec3::new(-half_size, half_size, half_size), color),
            // Back face
            (Vec3::new(-half_size, -half_size, -half_size), color),
            (Vec3::new(half_size, -half_size, -half_size), color),
            (Vec3::new(half_size, half_size, -half_size), color),
            (Vec3::new(-half_size, half_size, -half_size), color),
        ];

        // Define indices for the triangles
        let indices = vec![
            // Front face
            0, 1, 2, 0, 2, 3, // Back face
            4, 6, 5, 4, 7, 6, // Left face
            0, 3, 7, 0, 7, 4, // Right face
            1, 5, 6, 1, 6, 2, // Top face
            3, 2, 6, 3, 6, 7, // Bottom face
            0, 4, 5, 0, 5, 1,
        ];

        let mesh_builder = renderer
            .create_shape(vertices)
            .as_mesh()
            .with_indices(indices);

        renderer.create_object(mesh_builder, position, rotation, scale)
    }

    /// Create a sphere with the given properties
    pub fn create_sphere(
        renderer: &mut Renderer,
        radius: f32,
        segments: u32,
        rings: u32,
        color: Color,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Result<SceneObject, RendererError> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Create vertices
        for ring in 0..=rings {
            let phi = PI * (ring as f32) / (rings as f32);
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            for segment in 0..=segments {
                let theta = 2.0 * PI * (segment as f32) / (segments as f32);
                let sin_theta = theta.sin();
                let cos_theta = theta.cos();

                let x = radius * sin_phi * cos_theta;
                let y = radius * cos_phi;
                let z = radius * sin_phi * sin_theta;

                vertices.push((Vec3::new(x, y, z), color));
            }
        }

        // Create indices
        for ring in 0..rings {
            let ring_start = ring * (segments + 1);
            let next_ring_start = (ring + 1) * (segments + 1);

            for segment in 0..segments {
                // Upper triangle
                indices.push(ring_start + segment);
                indices.push(next_ring_start + segment);
                indices.push(next_ring_start + segment + 1);

                // Lower triangle
                indices.push(ring_start + segment);
                indices.push(next_ring_start + segment + 1);
                indices.push(ring_start + segment + 1);
            }
        }

        let mesh_builder = renderer
            .create_shape(vertices)
            .as_mesh()
            .with_indices(indices);

        renderer.create_object(mesh_builder, position, rotation, scale)
    }

    /// Create a cone with the given properties
    pub fn create_cone(
        renderer: &mut Renderer,
        radius: f32,
        height: f32,
        segments: u32,
        color: Color,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Result<SceneObject, RendererError> {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Apex vertex (at the top)
        vertices.push((Vec3::new(0.0, height / 2.0, 0.0), color));

        // Base vertices
        for i in 0..segments {
            let angle = 2.0 * PI * (i as f32) / (segments as f32);
            let x = radius * angle.cos();
            let z = radius * angle.sin();
            vertices.push((Vec3::new(x, -height / 2.0, z), color));
        }

        // Side faces
        for i in 0..segments {
            let next = (i + 1) % segments;
            indices.push(0); // Apex
            indices.push(i + 1);
            indices.push(next + 1);
        }

        // Base faces (triangulate the base)
        let center_idx = vertices.len();
        vertices.push((Vec3::new(0.0, -height / 2.0, 0.0), color));

        for i in 0..segments {
            let next = (i + 1) % segments;
            indices.push(center_idx.try_into().unwrap());
            indices.push(next + 1);
            indices.push(i + 1);
        }

        let mesh_builder = renderer
            .create_shape(vertices)
            .as_mesh()
            .with_indices(indices);

        renderer.create_object(mesh_builder, position, rotation, scale)
    }

    /// Create a plane (flat rectangle) with the given properties
    pub fn create_plane(
        renderer: &mut Renderer,
        width: f32,
        depth: f32,
        color: Color,
        position: Vec3,
        rotation: Quat,
        scale: Vec3,
    ) -> Result<SceneObject, RendererError> {
        let half_width = width / 2.0;
        let half_depth = depth / 2.0;

        let vertices = vec![
            (Vec3::new(-half_width, 0.0, -half_depth), color),
            (Vec3::new(half_width, 0.0, -half_depth), color),
            (Vec3::new(half_width, 0.0, half_depth), color),
            (Vec3::new(-half_width, 0.0, half_depth), color),
        ];

        let indices = vec![0, 2, 1, 0, 3, 2];

        let mesh_builder = renderer
            .create_shape(vertices)
            .as_mesh()
            .with_indices(indices);

        renderer.create_object(mesh_builder, position, rotation, scale)
    }
}
