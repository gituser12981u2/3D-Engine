use super::{
    common::{PrimitiveType, Vertex},
    shape_builders::MeshBuilder,
};
use metal::{Buffer, Device, MTLResourceOptions};

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
    pub primitive_type: PrimitiveType,
    #[allow(dead_code)]
    pub vertex_buffer: Buffer,
    #[allow(dead_code)]
    pub index_buffer: Option<Buffer>,
}

impl Mesh {
    pub fn new(device: &Device, mesh_builder: MeshBuilder) -> Self {
        let vertex_buffer = device.new_buffer_with_data(
            mesh_builder.vertices.as_ptr() as *const _,
            (mesh_builder.vertices.len() * std::mem::size_of::<Vertex>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeShared,
        );

        let index_buffer = mesh_builder.indices.as_ref().map(|indices| {
            device.new_buffer_with_data(
                indices.as_ptr() as *const _,
                (indices.len() * std::mem::size_of::<u32>()) as u64,
                MTLResourceOptions::CPUCacheModeDefaultCache
                    | MTLResourceOptions::StorageModeShared,
            )
        });

        Mesh {
            vertices: mesh_builder.vertices,
            indices: mesh_builder.indices,
            primitive_type: mesh_builder.primitive_type,
            vertex_buffer,
            index_buffer,
        }
    }
}

pub struct MeshStorage {
    meshes: Vec<Mesh>,
    device: Device,
}

impl MeshStorage {
    pub fn new(device: Device) -> Self {
        MeshStorage {
            meshes: Vec::new(),
            device,
        }
    }

    pub fn add_mesh(&mut self, mesh_builder: MeshBuilder) -> usize {
        let mesh = Mesh::new(&self.device, mesh_builder);
        self.meshes.push(mesh);
        self.meshes.len() - 1
    }

    pub fn get_mesh(&self, index: usize) -> Option<&Mesh> {
        self.meshes.get(index)
    }
}
