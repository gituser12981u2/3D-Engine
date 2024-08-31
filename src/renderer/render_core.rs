use super::{
    backend::GraphicsBackend,
    common::{BackendDrawCommand, IndexType, PrimitiveType, Vertex},
    mesh::{Mesh, MeshStorage},
    render_queue::DrawCommand,
    shape_builders::{
        shape_builder::{vec3_color_to_vertex, PrimitiveBuilder},
        MeshBuilder, TriangleBuilder,
    },
    Camera, Color, RendererError,
};
use crate::renderer::{
    backend::metal::MetalBackend, camera::CameraMovement, render_queue::RenderQueue,
};
use glam::{Quat, Vec3};
use std::{cell::RefCell, mem, rc::Rc};
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
        let device = backend.device().clone();

        // Debug information
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
            mesh_storage: MeshStorage::new(device),
            render_queue: RenderQueue::new(),
            window,
            camera,
            last_frame_time: std::time::Instant::now(),
        })
    }

    pub fn update(&mut self) {
        let view_projection_matrix =
            self.camera.get_projection_matrix() * self.camera.get_view_matrix();

        self.backend
            .update_uniform_buffer(&view_projection_matrix)
            .unwrap();

        // TODO: sort batches in an efficient manner
        // TODO: Implement Frustum Culling
    }

    pub fn render(&mut self) -> Result<(), RendererError> {
        let draw_commands = mem::take(&mut self.render_queue.draw_commands);

        for draw_command in draw_commands {
            match &draw_command {
                DrawCommand::Mesh { mesh_id, .. } => {
                    if let Some(mesh) = self.mesh_storage.get_mesh(*mesh_id) {
                        self.backend.update_vertex_buffer(&mesh.vertices)?;
                        if let Some(indices) = &mesh.indices {
                            self.backend.update_index_buffer(indices)?;
                        }
                    } else {
                        return Err(RendererError::InvalidMeshId);
                    }
                }
                DrawCommand::Primitive {
                    vertices, indices, ..
                } => {
                    self.backend.update_vertex_buffer(vertices)?;
                    if let Some(indices) = indices {
                        self.backend.update_index_buffer(indices)?;
                    }
                }
            }

            if let Some(instance_data) = draw_command.instance_data() {
                self.backend.update_instance_buffer(instance_data)?;
            }

            let backend_draw_command = self.create_backend_draw_command(&draw_command)?;
            self.backend.draw(backend_draw_command)?;
        }

        // Clear the render queue after drawing
        self.render_queue.clear();

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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn create_shape(&mut self, vertices: Vec<(Vec3, Color)>) -> PrimitiveBuilder {
        let vertices = vertices
            .into_iter()
            .map(|(pos, color)| vec3_color_to_vertex(pos, color))
            .collect();
        PrimitiveBuilder::new(vertices, PrimitiveType::Triangle)
    }

    pub fn create_mesh(&mut self, vertices: Vec<(Vec3, Color)>) -> MeshBuilder {
        let vertices = vertices
            .into_iter()
            .map(|(pos, color)| vec3_color_to_vertex(pos, color))
            .collect();
        MeshBuilder::new().with_vertices(vertices)
    }

    #[allow(dead_code)]
    pub fn create_pyramid(&mut self, base_width: f32, height: f32, color: [Color; 5]) -> usize {
        let half_width = base_width / 2.0;
        let vertices = vec![
            Vertex {
                position: [0.0, height, 0.0],
                color: color[0].into(),
            }, // Apex
            Vertex {
                position: [-half_width, 0.0, -half_width],
                color: color[1].into(),
            }, // Base 1
            Vertex {
                position: [half_width, 0.0, -half_width],
                color: color[2].into(),
            }, // Base 2
            Vertex {
                position: [half_width, 0.0, half_width],
                color: color[3].into(),
            }, // Base 3
            Vertex {
                position: [-half_width, 0.0, half_width],
                color: color[4].into(),
            }, // Base 4
        ];

        let indices = vec![
            0, 1, 2, // Front face
            0, 2, 3, // Right face
            0, 3, 4, // Back face
            0, 4, 1, // Left face
            1, 3, 2, // Base (part 1)
            1, 4, 3, // Base (part 2)
        ];

        let mesh_builder = MeshBuilder::new()
            .with_vertices(vertices)
            .with_indices(indices)
            .with_primitive_type(PrimitiveType::Triangle);

        self.add_mesh(mesh_builder)
    }

    #[deprecated(since = "0.1.0-alpha.2", note = "please use `create_triangle` instead")]
    #[allow(dead_code)]
    // Draw a triangle with the specified vertices and color
    pub fn draw_primitive_triangle(
        &mut self,
        v1: Vec3,
        v2: Vec3,
        v3: Vec3,
        color: Color,
    ) -> Result<(), RendererError> {
        let vertices = vec![
            Vertex {
                position: v1.to_array(),
                color: color.into(),
            },
            Vertex {
                position: v2.to_array(),
                color: color.into(),
            },
            Vertex {
                position: v3.to_array(),
                color: color.into(),
            },
        ];
        self.backend.update_vertex_buffer(&vertices)?;

        let draw_command = BackendDrawCommand::Basic {
            primitive_type: PrimitiveType::Triangle,
            vertex_start: 0,
            vertex_count: 3,
        };

        self.backend.draw(draw_command)?;
        Ok(())
    }

    #[deprecated(since = "0.1.0-alpha.2", note = "please use `create_shape` instead")]
    #[allow(dead_code)]
    // Draw a rectangle with the specified corners and color
    pub fn draw_primitive_rectangle(
        &mut self,
        top_left: Vec3,
        bottom_right: Vec3,
        color: Color,
    ) -> Result<(), RendererError> {
        let top_right = Vec3::new(bottom_right.x, top_left.y, top_left.z);
        let bottom_left = Vec3::new(top_left.x, bottom_right.y, top_left.z);
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
        // self.backend.draw_large_single_vertex(4, 6)

        let draw_command = BackendDrawCommand::Basic {
            primitive_type: PrimitiveType::Triangle,
            vertex_start: 0,
            vertex_count: 3,
        };

        self.backend.draw(draw_command)?;
        Ok(())
    }

    #[deprecated(since = "0.1.0-alpha.2", note = "please use `create_shape` instead")]
    #[allow(dead_code)]
    pub fn draw_primitive_pyramid(
        &mut self,
        base_width: f32,
        height: f32,
        position: Vec3,
        rotation: Quat,
        colors: [Color; 4],
    ) -> Result<(), RendererError> {
        let half_width = base_width / 2.0;
        let apex = position + Vec3::new(0.0, height, 0.0);
        let base1 = position + Vec3::new(-half_width, 0.0, -half_width);
        let base2 = position + Vec3::new(half_width, 0.0, -half_width);
        let base3 = position + Vec3::new(half_width, 0.0, half_width);
        let base4 = position + Vec3::new(-half_width, 0.0, half_width);

        // Rotate vertices
        let rotate = |v: Vec3| position + rotation * v;

        #[allow(deprecated)]
        self.draw_primitive_triangle(rotate(apex), rotate(base1), rotate(base2), colors[0])?; // Front

        #[allow(deprecated)]
        self.draw_primitive_triangle(rotate(apex), rotate(base2), rotate(base3), colors[1])?; // Right

        #[allow(deprecated)]
        self.draw_primitive_triangle(rotate(apex), rotate(base3), rotate(base4), colors[2])?; // Back

        #[allow(deprecated)]
        self.draw_primitive_triangle(rotate(apex), rotate(base4), rotate(base1), colors[3])?; // Left

        Ok(())
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
