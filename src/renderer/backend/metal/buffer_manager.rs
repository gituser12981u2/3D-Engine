use metal::{Buffer, Device};

use crate::renderer::{common::Vertex, RendererError};

pub struct BufferManager {
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
}

impl BufferManager {
    pub fn new() -> Self {
        BufferManager {
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    // TODO: pad the buffers to a single, ubiquitous size
    pub fn update_vertex_buffer(
        &mut self,
        device: &Device,
        vertices: &[Vertex],
    ) -> Result<(), RendererError> {
        println!(
            "Creating vertex buffer of size: {}",
            vertices.len() * std::mem::size_of_val(vertices)
        );
        let buffer_size = (vertices.len() * std::mem::size_of_val(vertices)) as u64;
        let new_buffer = device.new_buffer_with_data(
            vertices.as_ptr() as *const _,
            buffer_size,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache
                | metal::MTLResourceOptions::StorageModeShared,
        );
        self.vertex_buffer = Some(new_buffer);
        println!("Vertex buffer created");
        Ok(())
    }

    pub fn update_index_buffer(
        &mut self,
        device: &Device,
        indices: &[u32],
    ) -> Result<(), RendererError> {
        println!(
            "Creating index buffer of size: {}",
            indices.len() * std::mem::size_of_val(indices)
        );
        let buffer_size = (indices.len() * std::mem::size_of_val(indices)) as u64;
        let new_buffer = device.new_buffer_with_data(
            indices.as_ptr() as *const _,
            buffer_size,
            metal::MTLResourceOptions::CPUCacheModeDefaultCache
                | metal::MTLResourceOptions::StorageModeShared,
        );
        self.index_buffer = Some(new_buffer);
        println!("Index buffer created");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BufferManager;
    use crate::renderer::common::Vertex;
    use metal::Device;

    #[test]
    fn test_buffer_manager() {
        let device = Device::system_default().unwrap();
        let mut buffer_manager = BufferManager::new();

        let vertices = vec![
            Vertex {
                position: [0.0, 0.0, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ];

        let indices = vec![0, 1, 2];

        assert!(buffer_manager
            .update_vertex_buffer(&device, &vertices)
            .is_ok());
        assert!(buffer_manager
            .update_index_buffer(&device, &indices)
            .is_ok());

        // Check if the buffers have the correct size
        assert_eq!(
            buffer_manager.vertex_buffer.as_ref().unwrap().length(),
            (std::mem::size_of::<Vertex>() * vertices.len()) as u64
        );
        assert_eq!(
            buffer_manager.index_buffer.as_ref().unwrap().length(),
            (std::mem::size_of::<u32>() * indices.len()) as u64
        );

        // Check if the data was correctly written to the buffers
        unsafe {
            let vertex_data = std::slice::from_raw_parts(
                buffer_manager.vertex_buffer.as_ref().unwrap().contents() as *const Vertex,
                vertices.len(),
            );
            assert_eq!(vertex_data, vertices.as_slice());

            let index_data = std::slice::from_raw_parts(
                buffer_manager.index_buffer.as_ref().unwrap().contents() as *const u32,
                indices.len(),
            );
            assert_eq!(index_data, indices.as_slice());
        }
    }
}
