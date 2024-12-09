//! Render Queue Module
//!
//! The module provides structures and implementations for managing draw commands
//! in a rendering system. It includes types for instance data, draw commands,
//! and a render queue to manage these commands efficiently.

use super::{
    common::{PrimitiveType, Vertex},
    Color,
};
use crate::debug_trace;
use glam::Mat4;
use log::{debug, trace};

/// Represents instance-specific data for instanced rendering.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct InstanceData {
    pub model_matrix: Mat4,
    pub color: Color,
}

impl InstanceData {
    /// Creates a new `InstanceData`.
    ///
    /// # Arguments
    ///
    /// * `model_matrix` - The model matrix for this instance.
    /// * `color` - The color for this instance.
    #[allow(dead_code)]
    pub fn new(model_matrix: Mat4, color: Color) -> Self {
        Self {
            model_matrix,
            color,
        }
    }
}

/// Represents a draw command for the renderer.
#[derive(Clone)]
pub enum DrawCommand {
    Mesh {
        mesh_id: usize,
        instance_data: Option<Vec<InstanceData>>,
        transform: Mat4,
    },
    Primitive {
        vertices: Vec<Vertex>,
        indices: Option<Vec<u32>>,
        primitive_type: PrimitiveType,
        instance_data: Option<Vec<InstanceData>>,
        transform: Mat4,
    },
}

impl DrawCommand {
    /// Returns a reference to the instance data if present.
    pub fn instance_data(&self) -> Option<&Vec<InstanceData>> {
        match self {
            DrawCommand::Mesh { instance_data, .. }
            | DrawCommand::Primitive { instance_data, .. } => instance_data.as_ref(),
        }
    }
}

/// A builder for creating `DrawCommand's`.
pub struct DrawCommandBuilder {
    command: DrawCommand,
}

impl DrawCommandBuilder {
    /// Creates a new `DrawCommandBuilder` for a mesh.
    ///
    /// # Arguments
    ///
    /// * `mesh_id` - The ID of the mesh to draw.
    pub fn new_mesh(mesh_id: usize) -> Self {
        Self {
            command: DrawCommand::Mesh {
                mesh_id,
                instance_data: None,
                transform: Mat4::IDENTITY,
            },
        }
    }

    /// Creates a new `DrawCommandBuilder` for a primitive.
    ///
    /// # Arguments
    ///
    /// * `vertices` - The vertices of the primitive.
    /// * `indices` - Optional indices for indexed rendering.
    /// * `primitive_type` - The type of primitive to draw.
    pub fn new_primitive(
        vertices: Vec<Vertex>,
        indices: Option<Vec<u32>>,
        primitive_type: PrimitiveType,
    ) -> Self {
        Self {
            command: DrawCommand::Primitive {
                vertices,
                indices,
                primitive_type,
                instance_data: None,
                transform: Mat4::IDENTITY,
            },
        }
    }

    /// Sets the instance data to the draw command.
    ///
    /// # Arguments
    ///
    /// * `instance_data` - The instance data to add.
    pub fn with_instances(mut self, instance_data: Vec<InstanceData>) -> Self {
        match &mut self.command {
            DrawCommand::Mesh {
                instance_data: id, ..
            } => *id = Some(instance_data),
            DrawCommand::Primitive {
                instance_data: id, ..
            } => *id = Some(instance_data),
        }
        self
    }

    /// Sets the transformation matrix for the draw command.
    ///
    /// # Arguments
    ///
    /// * `transform` - The transformation matrix to apply.
    pub fn with_transform(mut self, transform: Mat4) -> Self {
        match &mut self.command {
            DrawCommand::Mesh { transform: t, .. } => *t = transform,
            DrawCommand::Primitive { transform: t, .. } => *t = transform,
        }
        self
    }

    /// Builds the `DrawCommand`.
    pub fn build(self) -> DrawCommand {
        self.command
    }
}

// TODO: Add batch calling back

/// Manages a queue of draw commands for rendering.
#[derive(Default)]
pub struct RenderQueue {
    pub draw_commands: Vec<DrawCommand>,
}

impl RenderQueue {
    /// Creates a new, empty `RenderQueue`.
    pub fn new() -> Self {
        debug!("Creating new RenderQueue");
        Self::default()
    }

    /// Adds a draw command to the queue.
    ///
    /// # Arguments
    ///
    /// * `command` - The draw command to add.
    pub fn add_draw_command(&mut self, command: DrawCommand) {
        debug_trace!("Adding draw command to RenderQueue");
        self.draw_commands.push(command);
    }

    /// Returns a slice of all draw commands in the queue.
    #[allow(dead_code)]
    pub fn get_draw_commands(&self) -> &[DrawCommand] {
        trace!("Retrieving draw commands from RenderQueue");
        &self.draw_commands
    }

    /// Sorts the batches in the render queue.
    ///
    /// This method is currently unimplemented and will panic if called.
    #[allow(dead_code)]
    pub fn sort_batches(&mut self) {
        // TODO: sort batches by material
        // TODO: sort batches front to back
        // TODO: sort batches back to front
        unimplemented!("Batch sorting is not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::{DrawCommand, DrawCommandBuilder, InstanceData, RenderQueue};
    use crate::renderer::{
        common::{PrimitiveType, Vertex},
        Color,
    };
    use glam::{Mat4, Vec3};

    #[test]
    fn test_render_queue_new() {
        let queue = RenderQueue::new();
        assert!(queue.draw_commands.is_empty());
    }

    #[test]
    fn test_render_queue_add_draw_command() {
        let mut queue = RenderQueue::new();
        let command = DrawCommand::Mesh {
            mesh_id: 1,
            instance_data: None,
            transform: Mat4::IDENTITY,
        };
        queue.add_draw_command(command.clone());
        assert_eq!(queue.draw_commands.len(), 1);
        assert!(matches!(
            queue.draw_commands[0],
            DrawCommand::Mesh { mesh_id: 1, .. }
        ))
    }

    #[test]
    fn test_render_queue_get_draw_commands() {
        let mut queue = RenderQueue::new();
        queue.add_draw_command(DrawCommand::Mesh {
            mesh_id: 1,
            instance_data: None,
            transform: Mat4::IDENTITY,
        });
        let commands = queue.get_draw_commands();
        assert_eq!(commands.len(), 1);
        assert!(matches!(commands[0], DrawCommand::Mesh { mesh_id: 1, .. }))
    }

    #[test]
    fn test_draw_command_builder_new_mesh() {
        let builder = DrawCommandBuilder::new_mesh(1);
        let command = builder.build();
        assert!(matches!(command, DrawCommand::Mesh { mesh_id: 1, .. }));
    }

    #[test]
    fn test_draw_command_builder_new_primitive() {
        let vertices = vec![Vertex::default()];
        let builder =
            DrawCommandBuilder::new_primitive(vertices.clone(), None, PrimitiveType::Triangle);
        let command = builder.build();
        assert!(matches!(
            command,
            DrawCommand::Primitive {
                primitive_type: PrimitiveType::Triangle,
                ..
            }
        ));
    }

    #[test]
    fn test_draw_command_builder_with_instances() {
        let instances = vec![InstanceData::new(
            Mat4::IDENTITY,
            Color::new(1.0, 0.0, 0.0, 1.0),
        )];
        let builder = DrawCommandBuilder::new_mesh(1).with_instances(instances.clone());
        let command = builder.build();
        assert!(
            matches!(command, DrawCommand::Mesh { instance_data: Some(data), .. } if data == instances),
        );
    }

    #[test]
    fn test_draw_command_builder_with_transform() {
        let transform = Mat4::from_scale(Vec3::new(2.0, 2.0, 2.0));
        let builder = DrawCommandBuilder::new_mesh(1).with_transform(transform);
        let command = builder.build();
        assert!(matches!(command, DrawCommand::Mesh { transform: t, .. } if t == transform));
    }
}
