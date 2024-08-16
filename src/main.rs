use std::time::Instant;

use common::vector3::RenderVector3;
use renderer::{Color, Renderer};

mod common;
mod physics;
mod renderer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut renderer = Renderer::new(800, 600, "Metal Renderer")?;
    let start_time = Instant::now();

    renderer.set_render_callback(move |r| {
        let elapsed = start_time.elapsed().as_secs_f32();

        r.draw_triangle(
            RenderVector3::new(0.0, 0.25 + 0.1 * elapsed.sin(), 0.0), // Top center
            RenderVector3::new(-0.25, -0.25, 0.0),                    // Bottom left
            RenderVector3::new(0.25, -0.25, 0.0),                     // Bottom right
            Color::new(1.0, 0.0, 0.0, 1.0),                           // Red color
        )?;

        // r.draw_rectangle(
        //     RenderVector3::new(-0.5, 0.5, 0.0),
        //     RenderVector3::new(0.5, -0.5, 0.0),
        //     Color::new(0.0, 1.0, 0.0, 1.0),
        // )?;

        Ok(())
    });

    renderer.run()?;
    Ok(())
}
