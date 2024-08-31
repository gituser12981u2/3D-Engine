use glam::{Mat4, Quat, Vec3};
use renderer::{shape_builders::shape_builder::ShapeBuilder, Color, InstanceData, RendererSystem};
use std::time::Instant;

mod physics;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer_system = RendererSystem::new(800, 600, "Metal Renderer")?;
    let start_time = Instant::now();

    renderer_system.set_render_callback(move |r| {
        // Examples:
        let elapsed = start_time.elapsed().as_secs_f32();
        // TODO: add documentation and implementation examples to documentation

        // Non-indexed, non-instanced primitive triangle
        r.create_triangle(
            Vec3::new(0.0, 0.5 + elapsed.sin(), 0.0), // Top center
            Vec3::new(-0.5, -0.5, 0.0),               // Bottom left
            Vec3::new(0.5, -0.5, 0.0),                // Bottom right
            Color::new(1.0, 0.0, 0.0, 1.0),           // Green color
        )
        .as_primitive()
        .with_transform(Mat4::from_translation(Vec3::new(1.5, 0.0, 0.0)))
        .draw(r);

        // Indexed primitive triangle
        r.create_triangle(
            Vec3::new(0.0, 0.5, 0.0),
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.5, -0.5, 0.0),
            Color::new(1.0, 0.0, 0.0, 1.0),
        )
        .as_primitive()
        .with_indices(vec![0, 1, 2])
        .with_transform(Mat4::from_translation(Vec3::new(1.5, 1.0, 0.0)))
        .draw(r);

        // Instanced primitive triangle
        let instance_data = vec![
            InstanceData::new(
                Mat4::from_translation(Vec3::new(-1.5, -1.0, 0.0)),
                Color::new(1.0, 0.0, 0.0, 1.0),
            ),
            InstanceData::new(
                Mat4::from_translation(Vec3::new(-1.5, -1.5, 0.0)),
                Color::new(1.0, 0.0, 0.0, 1.0),
            ),
            InstanceData::new(
                Mat4::from_translation(Vec3::new(-1.5, -2.0, 0.0)),
                Color::new(1.0, 0.0, 0.0, 1.0),
            ),
        ];
        r.create_triangle(
            Vec3::new(0.0, 0.25, 0.0),
            Vec3::new(-0.25, -0.25, 0.0),
            Vec3::new(0.25, -0.25, 0.0),
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .as_primitive()
        .with_instances(instance_data)
        .draw(r);

        // Custom shape
        let pentagon_vertices = vec![
            (Vec3::new(0.0, 0.5, 0.0), Color::new(1.0, 0.0, 0.0, 1.0)),
            (Vec3::new(-0.47, 0.15, 0.0), Color::new(0.0, 1.0, 0.0, 1.0)),
            (Vec3::new(-0.29, -0.4, 0.0), Color::new(0.0, 0.0, 1.0, 1.0)),
            (Vec3::new(0.29, -0.4, 0.0), Color::new(1.0, 1.0, 0.0, 1.0)),
            (Vec3::new(0.47, 0.15, 0.0), Color::new(0.0, 1.0, 1.0, 1.0)),
        ];
        r.create_shape(pentagon_vertices)
            .as_primitive()
            .with_indices(vec![0, 1, 2, 0, 2, 3, 0, 3, 4])
            .draw(r);

        // Non-indexed, non-instanced mesh triangle
        r.create_triangle(
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new(0.5, -0.5, 0.0),
            Vec3::new(0.0, 0.5, 0.0),
            Color::new(1.0, 0.0, 0.0, 1.0),
        )
        .as_mesh()
        .with_transform(Mat4::from_translation(Vec3::new(-1.5, 0.0, 0.0)))
        .draw(r);

        // Define pyramid dimensions
        let base_width = 1.0;
        let height = 1.5;
        let half_width = base_width / 2.0;

        // Define pyramid vertices
        let pyramid_vertices = vec![
            // Apex
            (Vec3::new(0.0, height, 0.0), Color::new(1.0, 0.0, 0.0, 1.0)),
            // Base vertices
            (
                Vec3::new(-half_width, 0.0, -half_width),
                Color::new(0.0, 1.0, 0.0, 1.0),
            ),
            (
                Vec3::new(half_width, 0.0, -half_width),
                Color::new(0.0, 0.0, 1.0, 1.0),
            ),
            (
                Vec3::new(half_width, 0.0, half_width),
                Color::new(1.0, 1.0, 0.0, 1.0),
            ),
            (
                Vec3::new(-half_width, 0.0, half_width),
                Color::new(0.0, 1.0, 1.0, 1.0),
            ),
        ];
        // Define indices for the pyramid faces
        let pyramid_indices = vec![
            0, 1, 2, // Front face
            0, 2, 3, // Right face
            0, 3, 4, // Back face
            0, 4, 1, // Left face
            1, 3, 2, // Base (part 1)
            1, 4, 3, // Base (part 2)
        ];
        r.create_shape(pyramid_vertices.clone())
            .with_indices(pyramid_indices.clone())
            .with_transform(Mat4::from_rotation_translation(
                Quat::from_rotation_y(elapsed),
                Vec3::new(0.0, 0.0, -3.0),
            ))
            .draw(r);

        // Create a pyramid mesh
        r.create_mesh(pyramid_vertices)
            .with_indices(pyramid_indices)
            .with_transform(Mat4::from_rotation_translation(
                Quat::from_rotation_y(elapsed),
                Vec3::new(0.0, 0.0, -3.0),
            ))
            .draw(r);

        r.update();
        r.render()
    });

    renderer_system.run()?;
    Ok(())
}
