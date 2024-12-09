use super::{
    backend::GraphicsBackend,
    common::{BackendDrawCommand, IndexType, PrimitiveType, Uniforms, Vertex},
    mesh::{Mesh, MeshStorage},
    render_queue::DrawCommand,
    shape_builders::{
        shape_builder::{vec3_color_to_vertex, ShapeData},
        MeshBuilder, TriangleBuilder,
    },
    Camera, Color, RendererError,
};
use crate::{
    debug_trace,
    renderer::{backend::metal::MetalBackend, camera::CameraMovement, render_queue::RenderQueue},
};
use glam::Vec3;
use log::info;
use std::{cell::RefCell, mem, rc::Rc, time::Instant};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyEvent, MouseScrollDelta, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

pub struct Renderer {
    backend: MetalBackend,
    mesh_storage: MeshStorage,
    render_queue: RenderQueue,
    // TODO: implement Material Manager and Scene Graph
    // material_manager: MaterialManager,
    // scene_graph: SceneGraph,
    window: Window,
    camera: Camera,
    last_frame_time: std::time::Instant,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ObjectId(pub usize);

impl Renderer {
    // Create a new Renderer with the specified window dimensions and title
    pub fn new(window: Window) -> Result<Self, RendererError> {
        let backend = MetalBackend::new(&window)?;
        // let device = backend.device().clone();
        let size = window.inner_size();

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, 3.0),
            45.0,
            size.width as f32 / size.height as f32,
            0.1,
            100.0,
        );

        Ok(Renderer {
            backend,
            mesh_storage: MeshStorage::new(),
            render_queue: RenderQueue::new(),
            window,
            camera,
            last_frame_time: std::time::Instant::now(),
        })
    }

    pub fn render(&mut self) -> Result<(), RendererError> {
        // TODO: sort batches in an efficient manner
        // TODO: Implement Frustum Culling

        let render_start = Instant::now();
        debug_trace!("Starting render at {:?}", render_start);

        let view_projection_matrix =
            self.camera.get_projection_matrix() * self.camera.get_view_matrix();

        // Implicitly clear the render queue by taking ownership of the draw commands
        let draw_commands = mem::take(&mut self.render_queue.draw_commands);
        debug_trace!("Clearing RenderQueue at {:?}", Instant::now());

        for draw_command in draw_commands {
            match &draw_command {
                DrawCommand::Mesh {
                    mesh_id, transform, ..
                } => {
                    if let Some(mesh) = self.mesh_storage.get_mesh(*mesh_id) {
                        self.backend.update_vertex_buffer(&mesh.vertices)?;
                        if let Some(indices) = &mesh.indices {
                            self.backend.update_index_buffer(indices)?;
                        }

                        let uniforms = Uniforms {
                            view_projection_matrix,
                            model_matrix: *transform,
                        };
                        self.backend.update_uniform_buffer(&uniforms).unwrap();
                    } else {
                        return Err(RendererError::InvalidMeshId);
                    }
                }
                DrawCommand::Primitive {
                    vertices,
                    indices,
                    transform,
                    ..
                } => {
                    self.backend.update_vertex_buffer(vertices)?;
                    if let Some(indices) = indices {
                        self.backend.update_index_buffer(indices)?;
                    }

                    let uniforms = Uniforms {
                        view_projection_matrix,
                        model_matrix: *transform,
                    };
                    self.backend.update_uniform_buffer(&uniforms)?;
                }
            }

            if let Some(instance_data) = draw_command.instance_data() {
                self.backend.update_instance_buffer(instance_data)?;
            }

            let backend_draw_command = self.create_backend_draw_command(&draw_command)?;
            self.backend.draw(backend_draw_command)?;
        }

        debug_trace!("Finished render at {:?}", Instant::now());
        Ok(())
    }

    fn create_backend_draw_command(
        &self,
        draw_command: &DrawCommand,
    ) -> Result<BackendDrawCommand, RendererError> {
        match draw_command {
            DrawCommand::Mesh { mesh_id, .. } => {
                if let Some(mesh) = self.mesh_storage.get_mesh(*mesh_id) {
                    Ok(self.create_backend_draw_command_from_mesh(mesh, draw_command))
                } else {
                    Err(RendererError::InvalidMeshId)
                }
            }
            DrawCommand::Primitive {
                vertices,
                indices,
                primitive_type,
                ..
            } => Ok(self.create_backend_draw_command_from_primitive(
                vertices,
                indices,
                primitive_type,
                draw_command,
            )),
        }
    }

    fn create_backend_draw_command_from_mesh(
        &self,
        mesh: &Mesh,
        draw_command: &DrawCommand,
    ) -> BackendDrawCommand {
        if let Some(instance_data) = draw_command.instance_data() {
            if mesh.indices.is_some() {
                BackendDrawCommand::IndexedInstanced {
                    primitive_type: mesh.primitive_type,
                    index_count: mesh.indices.as_ref().unwrap().len() as u64,
                    index_type: IndexType::UInt32,
                    index_buffer_offset: 0,
                    instance_count: instance_data.len() as u64,
                }
            } else {
                BackendDrawCommand::Instanced {
                    primitive_type: mesh.primitive_type,
                    vertex_start: 0,
                    vertex_count: mesh.vertices.len() as u64,
                    instance_count: instance_data.len() as u64,
                }
            }
        } else if mesh.indices.is_some() {
            BackendDrawCommand::Indexed {
                primitive_type: mesh.primitive_type,
                index_count: mesh.indices.as_ref().unwrap().len() as u64,
                index_type: IndexType::UInt32,
                index_buffer_offset: 0,
            }
        } else {
            BackendDrawCommand::Basic {
                primitive_type: mesh.primitive_type,
                vertex_start: 0,
                vertex_count: mesh.vertices.len() as u64,
            }
        }
    }

    pub fn create_backend_draw_command_from_primitive(
        &self,
        vertices: &[Vertex],
        indices: &Option<Vec<u32>>,
        primitive_type: &PrimitiveType,
        draw_command: &DrawCommand,
    ) -> BackendDrawCommand {
        if let Some(instance_data) = draw_command.instance_data() {
            if let Some(indices) = indices {
                BackendDrawCommand::IndexedInstanced {
                    primitive_type: *primitive_type,
                    index_count: indices.len() as u64,
                    index_type: IndexType::UInt32,
                    index_buffer_offset: 0,
                    instance_count: instance_data.len() as u64,
                }
            } else {
                BackendDrawCommand::Instanced {
                    primitive_type: *primitive_type,
                    vertex_start: 0,
                    vertex_count: vertices.len() as u64,
                    instance_count: instance_data.len() as u64,
                }
            }
        } else if let Some(indices) = indices {
            BackendDrawCommand::Indexed {
                primitive_type: *primitive_type,
                index_count: indices.len() as u64,
                index_type: IndexType::UInt32,
                index_buffer_offset: 0,
            }
        } else {
            BackendDrawCommand::Basic {
                primitive_type: *primitive_type,
                vertex_start: 0,
                vertex_count: vertices.len() as u64,
            }
        }
    }

    pub fn add_mesh(&mut self, mesh_builder: MeshBuilder) -> usize {
        self.mesh_storage.add_mesh(mesh_builder)
    }

    pub fn draw_immediate(&mut self, draw_command: DrawCommand) {
        self.render_queue.add_draw_command(draw_command);
    }

    // TODO: implement resize in the backend
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.camera
            .set_aspect_ratio(new_size.width as f32 / new_size.height as f32);
        // TODO: Update the backend
        // self.backend.resize(new_size);
    }

    // TODO: Find a way to tell rust these are exposed API methods so they should'nt be counted as dead code
    #[allow(dead_code)]
    pub fn create_triangle(
        &mut self,
        v1: Vec3,
        v2: Vec3,
        v3: Vec3,
        color: Color,
    ) -> TriangleBuilder {
        TriangleBuilder::new(v1, v2, v3, color)
    }

    pub fn create_shape(&mut self, vertices: Vec<(Vec3, Color)>) -> ShapeData {
        let vertices = vertices
            .into_iter()
            .map(|(position, color)| vec3_color_to_vertex(position, color))
            .collect();
        ShapeData::new(vertices, PrimitiveType::Triangle)
    }
}

pub type RenderCallback = dyn Fn(&mut Renderer) -> Result<(), RendererError>;

pub struct RendererSystem {
    renderer: Rc<RefCell<Renderer>>,
    event_loop: EventLoop<()>,
    render_callback: Box<RenderCallback>,
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
        info!("Initializing renderer system with {width}x{height} window");

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
                                        KeyCode::KeyV => renderer.backend.toggle_wireframe_mode(),
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
                                .unwrap();
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            let mut renderer = self.renderer.borrow_mut();
                            match delta {
                                MouseScrollDelta::LineDelta(_, y) => {
                                    renderer.camera.process_mouse_scroll(y);
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
                                eprintln!("Error in render callback: {e:?}");
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
