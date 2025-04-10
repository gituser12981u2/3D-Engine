use env_logger::Builder;
use glam::{Mat4, Quat, Vec3};
use log::LevelFilter;
use render_engine::{Color, RendererSystem, ShapeBuilder};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new().filter_level(LevelFilter::Debug).init();

    let mut renderer_system = RendererSystem::new(800, 600, "Metal Renderer")?;
    let start_time = Instant::now();

    renderer_system.set_render_callback(move |r| {
        let elapsed = start_time.elapsed().as_secs_f32();

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
        r.create_shape(pyramid_vertices)
            .as_mesh()
            .with_indices(pyramid_indices)
            .with_transform(Mat4::from_rotation_translation(
                Quat::from_rotation_y(elapsed),
                Vec3::new(0.0, -0.5, 0.0),
            ))
            .draw(r);

        r.render()
    });

    renderer_system.run()?;
    Ok(())
}
