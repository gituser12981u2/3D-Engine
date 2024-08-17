use std::time::Instant;

use common::vector3::RenderVector3;
use renderer::{Color, RendererSystem};

mod common;
mod physics;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer_system = RendererSystem::new(800, 600, "Metal Renderer")?;
    let start_time = Instant::now();

    renderer_system.set_render_callback(move |renderer| {
        let elapsed = start_time.elapsed().as_secs_f32();

        renderer.draw_triangle(
            RenderVector3::new(0.0, 0.5 * elapsed.sin(), 0.0), // Top center
            RenderVector3::new(-0.5, -0.5, 0.0),               // Bottom left
            RenderVector3::new(0.5, -0.5, 0.0),                // Bottom right
            Color::new(1.0, 0.0, 0.0, 1.0),                    // Red color
        )?;

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
