use super::{
    common::{PrimitiveType, Vertex},
    Color,
};
use glam::Mat4;

#[derive(Clone)]
pub struct InstanceData {
    #[allow(dead_code)]
    pub model_matrix: Mat4,
    #[allow(dead_code)]
    pub color: Color,
}

impl InstanceData {
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
