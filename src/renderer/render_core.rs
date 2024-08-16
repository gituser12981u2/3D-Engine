use super::{
    backend::{
        metal::{MacOSWindow, MetalBackend},
        GraphicsBackend,
    },
    common::{Color, RendererError, Vertex},
};
use crate::common::vector3::RenderVector3;
use std::{cell::RefCell, rc::Rc};

// Define a type alias for the render callback
type RenderCallback = dyn for<'a> FnMut(&'a mut Renderer) -> Result<(), RendererError>;

/// Renderer manages the rendering process and window
pub struct Renderer {
    backend: MetalBackend,
    window: MacOSWindow,
    // The render callback is a function that defines what to draw each frame
    render_callback: Rc<RefCell<RenderCallback>>,
}

impl Renderer {
    // Create a new Renderer with the specified window dimensions and title
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, RendererError> {
        let window = MacOSWindow::new(width, height, title)?;
        let backend = MetalBackend::new(&window)?;
        // Initialize with a no-op render callback
        let render_callback: Rc<RefCell<RenderCallback>> =
            Rc::new(RefCell::new(Box::new(|_: &mut Renderer| Ok(()))));
        Ok(Renderer {
            backend,
            window,
            render_callback,
        })
    }

    pub fn run(self) -> Result<(), RendererError> {
        let render_callback = self.render_callback.clone();
        let mut renderer = self; // Move self into a mutable variable
        let window = renderer.window.clone();
        window.run_loop(move |_event| {
            renderer.backend.prepare_frame()?;
            render_callback.borrow_mut()(&mut renderer)?;
            renderer.backend.present_frame()?;
            Ok(())
        })
    }

    // Set the function that will be called each frame to perform rendering
    pub fn set_render_callback<F>(&mut self, callback: F)
    where
        F: for<'a> FnMut(&'a mut Renderer) -> Result<(), RendererError> + 'static,
    {
        self.render_callback = Rc::new(RefCell::new(Box::new(callback)));
    }

    // Draw a triangle with the specified vertices and color
    pub fn draw_triangle(
        &mut self,
        v1: RenderVector3,
        v2: RenderVector3,
        v3: RenderVector3,
        color: Color,
    ) -> Result<(), RendererError> {
        let vertices = vec![
            Vertex {
                position: [v1.x, v1.y, v1.z],
                color: [color.r, color.g, color.b, color.a],
            },
            Vertex {
                position: [v2.x, v2.y, v2.z],
                color: [color.r, color.g, color.b, color.a],
            },
            Vertex {
                position: [v3.x, v3.y, v3.z],
                color: [color.r, color.g, color.b, color.a],
            },
        ];
        self.backend.update_vertex_buffer(&vertices)?;
        self.backend.draw(3, 0)
    }

    #[allow(dead_code)]
    // Draw a rectangle with the specified corners and color
    pub fn draw_rectangle(
        &mut self,
        top_left: RenderVector3,
        bottom_right: RenderVector3,
        color: Color,
    ) -> Result<(), RendererError> {
        let top_right = RenderVector3::new(bottom_right.x, top_left.y, top_left.z);
        let bottom_left = RenderVector3::new(top_left.x, bottom_right.y, top_left.z);
        let vertices = vec![
            Vertex {
                position: [top_left.x, top_left.y, top_left.z],
                color: [color.r, color.g, color.b, color.a],
            },
            Vertex {
                position: [top_right.x, top_right.y, top_right.z],
                color: [color.r, color.g, color.b, color.a],
            },
            Vertex {
                position: [bottom_left.x, bottom_left.y, bottom_left.z],
                color: [color.r, color.g, color.b, color.a],
            },
            Vertex {
                position: [bottom_right.x, bottom_right.y, bottom_right.z],
                color: [color.r, color.g, color.b, color.a],
            },
        ];
        let indices = vec![0, 1, 2, 1, 3, 2];
        self.backend.update_vertex_buffer(&vertices)?;
        self.backend.update_index_buffer(&indices)?;
        self.backend.draw(4, 6)
    }
}
