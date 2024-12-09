use std::num::NonZeroU32;

use metal::{Device, MTLRegion, Texture, TextureDescriptor};

use crate::renderer::{common::TextureId, RendererError};

pub struct TextureManager {
    device: Device,
    textures: Vec<Option<Texture>>,
}

impl TextureManager {
    pub fn new(device: &Device) -> Self {
        TextureManager {
            device: device.clone(),
            textures: Vec::new(),
        }
    }

    pub fn create_texture(&mut self, descriptor: &TextureDescriptor) -> TextureId {
        let texture = self.device.new_texture(descriptor);
        let id = TextureId(NonZeroU32::new(self.textures.len() as u32 + 1).unwrap());
        self.textures.push(Some(texture));
        id
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_texture(
        &self,
        id: TextureId,
        region: MTLRegion,
        mipmap_level: u64,
        slice: u64,
        bytes: &[u8],
        bytes_per_row: u64,
        bytes_per_image: u64,
    ) -> Result<(), RendererError> {
        if let Some(Some(texture)) = self.textures.get(id.0.get() as usize - 1) {
            texture.replace_region_in_slice(
                region,
                mipmap_level,
                slice,
                bytes.as_ptr() as *const std::ffi::c_void,
                bytes_per_row,
                bytes_per_image,
            );
            Ok(())
        } else {
            Err(RendererError::InvalidTextureId)
        }
    }
}
