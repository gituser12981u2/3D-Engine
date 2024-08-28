// use super::{material_manager::MaterialId, mesh_manager::MeshId, render_core::ObjectId};
// use glam::{Mat4, Quat, Vec3};

// pub struct SceneGraph {
//     object_ids: Vec<ObjectId>,
//     mesh_ids: Vec<MeshId>,
//     material_ids: Vec<MaterialId>,
//     positions: Vec<Vec3>,
//     rotations: Vec<Quat>,
//     scales: Vec<Vec3>,
//     // colors: Vec<Color>,
//     // pub mesh_indices: Vec<usize>,
// }

// impl SceneGraph {
//     pub fn new() -> Self {
//         SceneGraph {
//             object_ids: Vec::new(),
//             mesh_ids: Vec::new(),
//             material_ids: Vec::new(),
//             positions: Vec::new(),
//             rotations: Vec::new(),
//             scales: Vec::new(),
//         }
//     }

//     pub fn add_object(
//         &mut self,
//         mesh_id: MeshId,
//         material_id: MaterialId,
//         transform: Mat4,
//     ) -> ObjectId {
//         let object_id = ObjectId(self.object_ids.len());

//         let (scale, rotation, position) = transform.to_scale_rotation_translation();

//         self.object_ids.push(object_id);
//         self.mesh_ids.push(mesh_id);
//         self.material_ids.push(material_id);
//         self.positions.push(position);
//         self.rotations.push(rotation);
//         self.scales.push(scale);

//         object_id
//     }

//     pub fn update_object(&mut self, object_id: ObjectId, transform: Mat4) {
//         let index = self
//             .object_ids
//             .iter()
//             .position(|&id| id == object_id)
//             .unwrap();
//         let (scale, rotation, position) = transform.to_scale_rotation_translation();

//         self.positions[index] = position;
//         self.rotations[index] = rotation;
//         self.scales[index] = scale;
//     }

//     pub fn remove_object(&mut self, object_id: ObjectId) {
//         let index = self
//             .object_ids
//             .iter()
//             .position(|&id| id == object_id)
//             .unwrap();

//         self.object_ids.swap_remove(index);
//         self.mesh_ids.swap_remove(index);
//         self.material_ids.swap_remove(index);
//         self.positions.swap_remove(index);
//         self.rotations.swap_remove(index);
//         self.scales.swap_remove(index);
//     }
// }
