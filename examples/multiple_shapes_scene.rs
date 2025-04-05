use game_engine::{Color, RendererSystem, ShapeBuilder};
use glam::{Mat4, Quat, Vec3};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer_system = RendererSystem::new(800, 600, "Metal Renderer")?;

    renderer_system.set_render_callback(move |r| {
        // 1. Triangle using the high-level API
        r.create_triangle(
            Vec3::new(-1.5, 0.5, 0.0),
            Vec3::new(-2.0, -0.5, 0.0),
            Vec3::new(-1.0, -0.5, 0.0),
            Color::new(1.0, 0.0, 0.0, 1.0),
        )
        .as_primitive()
        .with_transform(Mat4::from_translation(Vec3::new(0.0, 0.0, 0.0)))
        .draw(r);

        // 2. Custom shape using the create_shape method
        let vertices = vec![
            // Create a square (position, color)
            (Vec3::new(0.5, 0.5, 0.0), Color::new(0.0, 1.0, 0.0, 1.0)),
            (Vec3::new(1.5, 0.5, 0.0), Color::new(0.0, 1.0, 0.0, 1.0)),
            (Vec3::new(1.5, -0.5, 0.0), Color::new(0.0, 1.0, 0.0, 1.0)),
            (Vec3::new(0.5, -0.5, 0.0), Color::new(0.0, 1.0, 0.0, 1.0)),
        ];

        let indices = vec![
            0, 1, 2, // First triangle
            0, 2, 3, // Second triangle
        ];

        r.create_shape(vertices)
            .as_mesh()
            .with_indices(indices)
            .with_transform(Mat4::from_rotation_translation(
                Quat::from_rotation_y(0.5),
                Vec3::new(0.0, 0.0, 0.0),
            ))
            .draw(r);

        r.render()
    });

    renderer_system.run()?;
    Ok(())
}
