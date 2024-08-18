use common::vector3::RenderVector3;
use renderer::{Color, RendererSystem};
use std::time::Instant;

mod common;
mod physics;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer_system = RendererSystem::new(800, 600, "Metal Renderer")?;
    let start_time = Instant::now();

    renderer_system.set_render_callback(move |renderer| {
        let elapsed = start_time.elapsed().as_secs_f32();
        let rotation = elapsed * 0.5; // Rotate the pyramid

        // Define the pyramid vertices
        let apex = RenderVector3::new(0.0, 0.5, 0.0);
        let base1 = RenderVector3::new(-0.5, -0.5, -0.5);
        let base2 = RenderVector3::new(0.5, -0.5, -0.5);
        let base3 = RenderVector3::new(0.5, -0.5, 0.5);
        let base4 = RenderVector3::new(-0.5, -0.5, 0.5);

        // Rotate vertices
        let rotate = |v: RenderVector3| {
            let x = v.x * rotation.cos() - v.z * rotation.sin();
            let z = v.x * rotation.sin() + v.z * rotation.cos();
            RenderVector3::new(x, v.y, z)
        };

        // Draw pyramid faces
        renderer.draw_triangle(
            rotate(apex),
            rotate(base1),
            rotate(base2),
            Color::new(1.0, 0.0, 0.0, 1.0),
        )?; // Front
        renderer.draw_triangle(
            rotate(apex),
            rotate(base2),
            rotate(base3),
            Color::new(0.0, 1.0, 0.0, 1.0),
        )?; // Right
        renderer.draw_triangle(
            rotate(apex),
            rotate(base3),
            rotate(base4),
            Color::new(0.0, 0.0, 1.0, 1.0),
        )?; // Back
        renderer.draw_triangle(
            rotate(apex),
            rotate(base4),
            rotate(base1),
            Color::new(1.0, 1.0, 0.0, 1.0),
        )?; // Left
            // renderer.draw_triangle(apex, base1, base2, Color::new(1.0, 0.0, 0.0, 1.0))?; // Front
            // renderer.draw_triangle(apex, base2, base3, Color::new(0.0, 1.0, 0.0, 1.0))?; // Right
            // renderer.draw_triangle(apex, base3, base4, Color::new(0.0, 0.0, 1.0, 1.0))?; // Back
            // renderer.draw_triangle(apex, base4, base1, Color::new(1.0, 1.0, 0.0, 1.0))?; // Left

        // renderer.draw_grid(20.0, 20)?;

        // renderer.draw_triangle(
        //     RenderVector3::new(0.0, 0.5 + elapsed.sin(), 0.0), // Top center
        //     RenderVector3::new(-0.5, -0.5, 0.0),               // Bottom left
        //     RenderVector3::new(0.5, -0.5, 0.0),                // Bottom right
        //     Color::new(1.0, 0.0, 0.0, 1.0),                    // Red color
        // )?;

        // renderer.draw_rectangle(
        //     RenderVector3::new(-0.5, 0.5, 0.0),
        //     RenderVector3::new(0.5, -0.5, 0.0),
        //     Color::new(0.0, 1.0, 0.0, 1.0),
        // )?;

        Ok(())
    });

    renderer_system.run()?;
    Ok(())
}
