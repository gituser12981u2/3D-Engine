use std::time::Instant;

use env_logger::Builder;
use glam::{Quat, Vec3};
use log::LevelFilter;
use render_engine::{renderer::ShapeFactory, Color, RendererSystem, ShapeBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new().filter_level(LevelFilter::Debug).init();

    let mut renderer_system = RendererSystem::new(800, 600, "Scene Graph Demo")?;
    let start_time = Instant::now();

    let mut pyramid = None;
    let mut cube = None;

    renderer_system.set_render_callback(move |renderer| {
        let elapsed = start_time.elapsed().as_secs_f32();

        if pyramid.is_none() {
            // Create a pyramid
            let pyramid_vertices = vec![
                // Apex
                (Vec3::new(0.0, 1.5, 0.0), Color::new(1.0, 0.0, 0.0, 1.0)),
                // Base vertices
                (Vec3::new(-0.5, 0.0, -0.5), Color::new(0.0, 1.0, 0.0, 1.0)),
                (Vec3::new(0.5, 0.0, -0.5), Color::new(0.0, 0.0, 1.0, 1.0)),
                (Vec3::new(0.5, 0.0, 0.5), Color::new(1.0, 1.0, 0.0, 1.0)),
                (Vec3::new(-0.5, 0.0, 0.5), Color::new(0.0, 1.0, 1.0, 1.0)),
            ];

            let pyramid_indices = vec![
                0, 1, 2, // Front face
                0, 2, 3, // Right face
                0, 3, 4, // Back face
                0, 4, 1, // Left face
                1, 3, 2, // Base (part 1)
                1, 4, 3, // Base (part 2)
            ];

            // Create the mesh builder and add to scene
            let pyramid_mesh = renderer
                .create_shape(pyramid_vertices)
                .as_mesh()
                .with_indices(pyramid_indices);

            pyramid = Some(renderer.create_object(
                pyramid_mesh,
                Vec3::new(-2.0, 0.0, 0.0),
                Quat::IDENTITY,
                Vec3::ONE,
            )?);

            // Create a cube
            cube = Some(ShapeFactory::create_cube(
                renderer,
                1.0,
                Color::new(0.2, 0.5, 0.8, 1.0),
                Vec3::new(0.0, 0.5, 0.0),
                Quat::IDENTITY,
                Vec3::ONE,
            )?);
        }

        if let Some(ref pyramid_obj) = pyramid {
            pyramid_obj.set_rotation(renderer, Quat::from_rotation_y(elapsed * 1.0))?;
        }

        if let Some(ref obj) = cube {
            obj.set_rotation(
                renderer,
                Quat::from_rotation_y(elapsed * 0.7) * Quat::from_rotation_x(elapsed * 0.5),
            )?;
        }

        // r.create_shape(pyramid_vertices)
        //     .as_mesh()
        //     .with_indices(pyramid_indices)
        //     .with_transform(Mat4::from_rotation_translation(
        //         Quat::from_rotation_y(elapsed),
        //         Vec3::new(0.0, -0.5, 0.0),
        //     ))
        //     .draw(r);

        renderer.render()
    });

    renderer_system.run()?;
    Ok(())
}
