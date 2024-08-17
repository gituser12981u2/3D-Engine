use super::{
    backend::{metal::MetalBackend, GraphicsBackend},
    common::{Color, RendererError, Vertex},
    Camera,
};
use crate::common::vector3::RenderVector3;
use glam::Vec3;
use std::{cell::RefCell, rc::Rc};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

/// Renderer manages the rendering process and window
pub struct Renderer {
    backend: MetalBackend,
    window: Window,
    camera: Camera,
}

// Define a type alias for the render callback
pub type RenderCallback = dyn Fn(&mut Renderer) -> Result<(), RendererError>;

pub struct RendererSystem {
    renderer: Rc<RefCell<Renderer>>,
    event_loop: EventLoop<()>,
    render_callback: Box<RenderCallback>,
}

impl Renderer {
    // Create a new Renderer with the specified window dimensions and title
    pub fn new(window: Window) -> Result<Self, RendererError> {
        let backend = MetalBackend::new(&window)?;
        let size = window.inner_size();
        let scale_factor = window.scale_factor();
        println!("Window size: {:?}, Scale factor: {scale_factor}", size);

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 3.0),
            std::f32::consts::PI / 4.0,
            size.width as f32 / size.height as f32,
            0.1,
            100.0,
        );

        Ok(Renderer {
            backend,
            window,
            camera,
        })
    }

    // TODO: implement resize in the backend
    // pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
    //     self.camera
    //         .set_aspect_ratio(new_size.width as f32 / new_size.height as f32);
    //     // Update the backend if necessary
    //     self.backend.resize(new_size);
    // }

    pub fn render(&mut self) -> Result<(), RendererError> {
        let view_matrix = self.camera.view_matrix();
        let projection_matrix = self.camera.projection_matrix();
        let view_projection_matrix = projection_matrix * view_matrix;

        println!("View-Projection Matrix: {:?}", view_projection_matrix);

        self.backend.update_uniforms(view_projection_matrix);

        self.backend.prepare_frame()?;
        self.backend.present_frame()?;

        Ok(())
    }

    // pub fn set_camera_position(&mut self, position: Vec3) {
    //     self.camera.set_position(position);
    // }

    // pub fn set_camera_orientation(&mut self, orientation: Quat) {
    //     self.camera.set_orientation(orientation);
    // }

    // pub fn move_camera(&mut self, direction: Vec3) {
    //     self.camera.move_camera(direction);
    // }

    // pub fn rotate_camera(&mut self, pitch: f32, yaw: f32) {
    //     self.camera.rotate_camera(pitch, yaw);
    // }

    /// Draw a triangle with the specified vertices and color
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
    /// Draw a rectangle with the specified corners and color
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

impl RendererSystem {
    pub fn new(width: u32, height: u32, title: &str) -> Result<Self, RendererError> {
        let event_loop =
            EventLoop::new().map_err(|e| RendererError::EventLoopError(e.to_string()))?;

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .map_err(|e| RendererError::WindowCreationFailed(e.to_string()))?;

        let renderer = Rc::new(RefCell::new(Renderer::new(window)?));

        Ok(RendererSystem {
            renderer,
            event_loop,
            render_callback: Box::new(|_| Ok(())), // Default no-op callback
        })
    }

    pub fn set_render_callback<F>(&mut self, callback: F)
    where
        F: Fn(&mut Renderer) -> Result<(), RendererError> + 'static,
    {
        self.render_callback = Box::new(callback);
    }

    pub fn run(self) -> Result<(), RendererError> {
        let RendererSystem {
            renderer,
            event_loop,
            render_callback,
        } = self;

        event_loop
            .run(move |event, event_loop_window_target| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    event_loop_window_target.exit();
                }
                // Event::WindowEvent {
                //     event: WindowEvent::Resized(new_size),
                //     ..
                // } => {
                //     renderer.borrow_mut().resize(new_size);
                // }
                Event::AboutToWait => {
                    renderer.borrow().window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // if let Err(e) = self.backend.prepare_frame() {
                    //     eprintln!("Error preparing frame: {:?}", e);
                    // }
                    // if let Err(e) = self.render_callback.borrow_mut()(&mut self) {
                    //     eprintln!("Error in render callback: {:?}", e);
                    // }
                    // if let Err(e) = self.backend.present_frame() {
                    //     eprintln!("Error presenting frame: {:?}", e);
                    // }

                    if let Err(e) = render_callback(&mut renderer.borrow_mut()) {
                        eprintln!("Error in render callback: {:?}", e);
                    }

                    if let Err(e) = renderer.borrow_mut().render() {
                        eprintln!("Error rendering: {:?}", e);
                    }
                }
                _ => (),
            })
            .map_err(|e| RendererError::EventLoopError(e.to_string()))
    }
}
