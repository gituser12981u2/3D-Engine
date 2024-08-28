// use super::common::{RenderPipelineId, TextureId};

// pub struct MaterialId(usize);

// pub struct UniformData {
//     // Placeholder for uniform data
//     // This could be a more complex structure depending on your needs
//     data: Vec<u8>,
// }

// pub struct Material {
//     pub pipeline_id: RenderPipelineId,
//     pub texture_id: Option<TextureId>,
//     pub uniform_data: UniformData,
// }

// pub struct MaterialManager {
//     pipeline_ids: Vec<RenderPipelineId>,
//     textures: Vec<Option<TextureId>>,
//     uniforms: Vec<UniformData>,
// }

// impl MaterialManager {
//     pub fn new() -> Self {
//         MaterialManager {
//             pipeline_ids: Vec::new(),
//             textures: Vec::new(),
//             uniforms: Vec::new(),
//         }
//     }

//     pub fn create_material(
//         &mut self,
//         pipeline_id: RenderPipelineId,
//         texture_id: Option<TextureId>,
//         uniform_data: UniformData,
//     ) -> MaterialId {
//         let material_id = MaterialId(self.pipeline_ids.len());

//         self.pipeline_ids.push(pipeline_id);
//         self.textures.push(texture_id);
//         self.uniforms.push(uniform_data);

//         material_id
//     }

//     pub fn get(&self, id: MaterialId) -> Material {
//         Material {
//             pipeline_id: self.pipeline_ids[id.0],
//             texture_id: self.textures[id.0],
//             uniform_data: self.uniforms[id.0].clone(),
//         }
//     }
// }
