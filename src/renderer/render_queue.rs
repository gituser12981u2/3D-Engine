use super::{
    common::{PrimitiveType, Vertex},
    Color,
};
use glam::Mat4;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct InstanceData {
    pub model_matrix: Mat4,
    pub color: Color,
}

impl InstanceData {
    #[allow(dead_code)]
    pub fn new(model_matrix: Mat4, color: Color) -> Self {
        InstanceData {
            model_matrix,
            color,
        }
    }
}

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
    pub fn instance_data(&self) -> Option<&Vec<InstanceData>> {
        match self {
            DrawCommand::Mesh { instance_data, .. } => instance_data.as_ref(),
            DrawCommand::Primitive { instance_data, .. } => instance_data.as_ref(),
        }
    }
}

pub struct DrawCommandBuilder {
    command: DrawCommand,
}

impl DrawCommandBuilder {
    pub fn new_mesh(mesh_id: usize) -> Self {
        DrawCommandBuilder {
            command: DrawCommand::Mesh {
                mesh_id,
                instance_data: None,
                transform: Mat4::IDENTITY,
            },
        }
    }

    pub fn new_primitive(
        vertices: Vec<Vertex>,
        indices: Option<Vec<u32>>,
        primitive_type: PrimitiveType,
    ) -> Self {
        DrawCommandBuilder {
            command: DrawCommand::Primitive {
                vertices,
                indices,
                primitive_type,
                instance_data: None,
                transform: Mat4::IDENTITY,
            },
        }
    }

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

    pub fn with_transform(mut self, transform: Mat4) -> Self {
        match &mut self.command {
            DrawCommand::Mesh { transform: t, .. } => *t = transform,
            DrawCommand::Primitive { transform: t, .. } => *t = transform,
        }
        self
    }

    // TODO: reimplement
    // pub fn with_color(mut self, color: Color) -> Self {
    //     self.color = color;
    //     self
    // }

    pub fn build(self) -> DrawCommand {
        self.command
    }
}

// pub struct RenderBatch {
//     pub mesh_id: usize,
//     pub start_instance: usize,
//     pub instance_count: usize,
// }

pub struct RenderQueue {
    // instances: Vec<InstanceData>,
    // batches: Vec<RenderBatch>,
    pub draw_commands: Vec<DrawCommand>,
}

impl RenderQueue {
    pub fn new() -> Self {
        RenderQueue {
            // instances: Vec::new(),
            // batches: Vec::new(),
            draw_commands: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        // self.instances.clear();
        // self.batches.clear();
        self.draw_commands.clear();
    }

    pub fn add_draw_command(&mut self, command: DrawCommand) {
        self.draw_commands.push(command);
    }

    #[allow(dead_code)]
    pub fn get_draw_commands(&self) -> &[DrawCommand] {
        &self.draw_commands
    }

    // pub fn add_instance(&mut self, mesh_id: usize, model_matrix: Mat4, color: Color) {
    //     let instance_data = InstanceData {
    //         model_matrix,
    //         color,
    //     };

    //     if let Some(batch) = self.batches.last_mut() {
    //         if batch.mesh_id == mesh_id {
    //             batch.instance_count += 1;
    //             self.instances.push(instance_data);
    //             return;
    //         }
    //     }

    //     self.batches.push(RenderBatch {
    //         mesh_id,
    //         start_instance: self.instances.len(),
    //         instance_count: 1,
    //     });
    //     self.instances.push(instance_data);
    // }

    #[allow(dead_code)]
    pub fn sort_batches(&mut self) {
        // TODO: sort batches by material
        // TODO: sort batches front to back
        // TODO: sort batches back to front
        unimplemented!()
    }

    // pub fn get_batches(&self) -> &[RenderBatch] {
    //     &self.batches
    // }

    // pub fn get_instances(&self) -> &[InstanceData] {
    //     &self.instances
    // }
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
    fn test_render_queue_clear() {
        let mut queue = RenderQueue::new();
        queue.add_draw_command(DrawCommand::Mesh {
            mesh_id: 1,
            instance_data: None,
            transform: Mat4::IDENTITY,
        });
        queue.clear();
        assert!(queue.draw_commands.is_empty());
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
