// use super::common::{BufferId, IndexType, PrimitiveType, Vertex};

// pub struct MeshId(usize);

// pub struct Mesh {
//     pub vertex_buffer_id: BufferId,
//     pub index_buffer_id: BufferId,
//     pub primitive_type: PrimitiveType,
//     pub index_count: u64,
//     pub index_type: IndexType,
// }

// pub struct MeshManager {
//     vertices: Vec<Vec<Vertex>>,
//     indices: Vec<Vec<u32>>,
//     vertex_buffer_ids: Vec<BufferId>,
//     index_buffer_ids: Vec<BufferId>,
//     primitive_types: Vec<PrimitiveType>,
//     index_counts: Vec<u64>,
//     index_types: Vec<IndexType>,
// }

// impl MeshManager {
//     pub fn new() -> Self {
//         MeshManager {
//             vertices: Vec::new(),
//             indices: Vec::new(),
//             vertex_buffer_ids: Vec::new(),
//             index_buffer_ids: Vec::new(),
//             primitive_types: Vec::new(),
//             index_counts: Vec::new(),
//             index_types: Vec::new(),
//         }
//     }

//     pub fn create_mesh(
//         &mut self,
//         vertices: Vec<Vertex>,
//         indices: Vec<u32>,
//         primitive_type: PrimitiveType,
//     ) -> MeshId {
//         let mesh_id = MeshId(self.vertices.len());

//         self.vertices.push(vertices);
//         self.indices.push(indices);
//         self.primitive_types.push(primitive_type);
//         self.index_counts
//             .push(self.indices.last().unwrap().len() as u64);
//         self.index_types.push(IndexType::UInt32); // Assuming 32-bit indices

//         // Note: In a real implementation, we would create GPU buffers here
//         self.vertex_buffer_ids.push(BufferId(0)); // Placeholder
//         self.index_buffer_ids.push(BufferId(0)); // Placeholder

//         mesh_id
//     }

//     pub fn get(&self, id: MeshId) -> Mesh {
//         Mesh {
//             vertex_buffer_id: self.vertex_buffer_ids[id.0],
//             index_buffer_id: self.index_buffer_ids[id.0],
//             primitive_type: self.primitive_types[id.0],
//             index_count: self.index_counts[id.0],
//             index_type: self.index_types[id.0],
//         }
//     }

//     // Add methods for updating and removing meshes as needed
// }
