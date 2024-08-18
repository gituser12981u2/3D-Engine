use super::{
    backend::{metal::MetalBackend, GraphicsBackend},
    common::{Color, RendererError, Vertex},
    Camera,
};
use crate::{common::vector3::RenderVector3, renderer::camera::CameraMovement};
use glam::Vec3;
use std::{cell::RefCell, rc::Rc};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyEvent, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

/// Renderer manages the rendering process and window
pub struct Renderer {
    backend: MetalBackend,
    window: Window,
    camera: Camera,
    last_frame_time: std::time::Instant,
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
            45.0,
            size.width as f32 / size.height as f32,
            0.1,
            100.0,
        );

        Ok(Renderer {
            backend,
            window,
            camera,
            last_frame_time: std::time::Instant::now(),
        })
    }

    pub fn render(&mut self) -> Result<(), RendererError> {
        let view_matrix = self.camera.get_view_matrix();
        let projection_matrix = self.camera.get_projection_matrix();
        let view_projection_matrix = projection_matrix * view_matrix;

        println!("View-Projection Matrix: {:?}", view_projection_matrix);

        self.backend.update_uniforms(view_projection_matrix);

        self.backend.prepare_frame()?;
        self.backend.present_frame()?;

        Ok(())
    }

    // TODO: implement resize in the backend
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.camera
            .set_aspect_ratio(new_size.width as f32 / new_size.height as f32);
        // Update the backend if necessary
        // self.backend.resize(new_size);
    }

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

    #[allow(dead_code)]
    pub fn draw_grid(&mut self, size: f32, divisions: u32) -> Result<(), RendererError> {
        let step = size / divisions as f32;
        let half_size = size / 2.0;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Create grid lines
        for i in 0..=divisions {
            let pos = -half_size + i as f32 * step;

            // X-axis line
            vertices.push(Vertex {
                position: [-half_size, 0.0, pos],
                color: [0.5, 0.5, 0.5, 1.0],
            });
            vertices.push(Vertex {
                position: [half_size, 0.0, pos],
                color: [0.5, 0.5, 0.5, 1.0],
            });

            // Z-axis line
            vertices.push(Vertex {
                position: [pos, 0.0, -half_size],
                color: [0.5, 0.5, 0.5, 1.0],
            });
            vertices.push(Vertex {
                position: [pos, 0.0, half_size],
                color: [0.5, 0.5, 0.5, 1.0],
            });

            // Create indices for lines
            let base_index = i * 4;
            indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index + 3,
            ]);
        }

        self.backend.update_vertex_buffer(&vertices)?;
        self.backend.update_index_buffer(&indices)?;
        self.backend
            .draw(vertices.len() as u32, indices.len() as u32)
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
        let window_size = self.renderer.borrow().window.inner_size();
        let center_x = window_size.width as f64 / 2.0;
        let center_y = window_size.height as f64 / 2.0;

        // Enable mouse capture
        self.renderer
            .borrow()
            .window
            .set_cursor_grab(winit::window::CursorGrabMode::Confined)
            .or(self
                .renderer
                .borrow()
                .window
                .set_cursor_grab(winit::window::CursorGrabMode::Locked))
            .unwrap();
        self.renderer.borrow().window.set_cursor_visible(false);

        self.event_loop
            .run(move |event, event_loop_window_target| {
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => event_loop_window_target.exit(),
                        WindowEvent::Resized(new_size) => {
                            self.renderer.borrow_mut().resize(new_size);
                        }
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    physical_key,
                                    state,
                                    ..
                                },
                            ..
                        } => {
                            let mut renderer = self.renderer.borrow_mut();
                            let mut delta_time = renderer.last_frame_time.elapsed().as_secs_f32();
                            delta_time = delta_time.min(0.1); // Cap delta_time to 0.1 seconds
                            renderer.last_frame_time = std::time::Instant::now();

                            if let PhysicalKey::Code(key_code) = physical_key {
                                use winit::event::ElementState;
                                if state == ElementState::Pressed {
                                    match key_code {
                                        KeyCode::KeyW => renderer
                                            .camera
                                            .process_keyboard(CameraMovement::Forward, delta_time),
                                        KeyCode::KeyS => renderer
                                            .camera
                                            .process_keyboard(CameraMovement::Backward, delta_time),
                                        KeyCode::KeyA => renderer
                                            .camera
                                            .process_keyboard(CameraMovement::Left, delta_time),
                                        KeyCode::KeyD => renderer
                                            .camera
                                            .process_keyboard(CameraMovement::Right, delta_time),
                                        KeyCode::Space => renderer
                                            .camera
                                            .process_keyboard(CameraMovement::Up, delta_time),
                                        KeyCode::ShiftLeft => renderer
                                            .camera
                                            .process_keyboard(CameraMovement::Down, delta_time),
                                        _ => {}
                                    }
                                }
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            let mut renderer = self.renderer.borrow_mut();

                            let delta_x = position.x - center_x;
                            let delta_y = center_y - position.y; // Reversed since y-coordinates go from bottom to top
                            renderer
                                .camera
                                .process_mouse_movement(delta_x as f32, delta_y as f32);

                            // Reset cursor position to center
                            renderer
                                .window
                                .set_cursor_position(winit::dpi::PhysicalPosition::new(
                                    center_x, center_y,
                                ))
                                .unwrap()
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let mut renderer = self.renderer.borrow_mut();
                            match delta {
                                MouseScrollDelta::LineDelta(_, y) => {
                                    renderer.camera.process_mouse_scroll(y)
                                }
                                MouseScrollDelta::PixelDelta(position) => renderer
                                    .camera
                                    .process_mouse_scroll(position.y as f32 * 0.1),
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            let mut renderer = self.renderer.borrow_mut();

                            // Draw objects
                            if let Err(e) = (self.render_callback)(&mut renderer) {
                                eprintln!("Error in render callback: {:?}", e);
                            }

                            // Present the frame
                            if let Err(e) = renderer.render() {
                                eprintln!("Error rendering: {:?}", e);
                            }
                        }
                        _ => {}
                    },
                    Event::AboutToWait => {
                        self.renderer.borrow().window.request_redraw();
                    }
                    _ => {}
                }
            })
            .map_err(|e| RendererError::EventLoopError(e.to_string()))
    }
}
