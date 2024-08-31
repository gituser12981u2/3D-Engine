use crate::renderer::{
    common::{BufferId, Vertex},
    render_queue::InstanceData,
    RendererError,
};
use glam::Mat4;
use metal::{Buffer, Device, MTLResourceOptions};

// TODO: find a more dynamic way of setting max buffer sizes
// Constants for maximum buffer size
const MAX_VERTICES: usize = 65536; // 2^16
const MAX_INDICES: usize = 196608; // 65536 * 3
const MAX_INSTANCES: usize = 4096;

pub struct BufferManager {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub uniform_buffer: Buffer,
    pub instance_buffer: Buffer,
    vertex_count: usize,
    index_count: usize,
    instance_count: usize,
}

impl BufferManager {
    pub fn new(device: &Device) -> Result<Self, RendererError> {
        let vertex_buffer = device.new_buffer(
            (std::mem::size_of::<Vertex>() * MAX_VERTICES) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeShared,
        );

        let index_buffer = device.new_buffer(
            (std::mem::size_of::<u32>() * MAX_INDICES) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeShared,
        );

        let uniform_buffer = device.new_buffer(
            std::mem::size_of::<Mat4>() as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        );

        let instance_buffer = device.new_buffer(
            (std::mem::size_of::<InstanceData>() * MAX_INSTANCES) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeShared,
        );

        Ok(BufferManager {
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            instance_buffer,
            vertex_count: 0,
            index_count: 0,
            instance_count: 0,
        })
    }

    pub fn update_vertex_buffer(&mut self, vertices: &[Vertex]) -> Result<(), RendererError> {
        if vertices.len() > MAX_VERTICES {
            return Err(RendererError::BufferOverflow);
        }

        unsafe {
            let dest: *mut Vertex = self.vertex_buffer.contents() as *mut Vertex;
            std::ptr::copy_nonoverlapping(vertices.as_ptr(), dest, vertices.len());
        }

        self.vertex_count = vertices.len();
        println!("Vertex buffer updated with {} vertices", self.vertex_count);
        Ok(())
    }

    pub fn update_index_buffer(&mut self, indices: &[u32]) -> Result<(), RendererError> {
        if indices.len() > MAX_INDICES {
            return Err(RendererError::BufferOverflow);
        }

        unsafe {
            let dest: *mut u32 = self.index_buffer.contents() as *mut u32;
            std::ptr::copy_nonoverlapping(indices.as_ptr(), dest, indices.len());
        }

        self.index_count = indices.len();
        println!("Index buffer updated with {} indices", self.index_count);
        Ok(())
    }

    pub fn update_uniform_buffer(&mut self, uniform_data: &Mat4) -> Result<(), RendererError> {
        unsafe {
            let dest: *mut Mat4 = self.uniform_buffer.contents() as *mut Mat4;
            *dest = *uniform_data;
        }
        self.uniform_buffer.did_modify_range(metal::NSRange {
            location: 0,
            length: std::mem::size_of::<Mat4>() as u64,
        });

        Ok(())
    }

    pub fn update_instance_buffer(
        &mut self,
        instances: &[InstanceData],
    ) -> Result<(), RendererError> {
        if instances.len() > MAX_INSTANCES {
            return Err(RendererError::BufferOverflow);
        }

        unsafe {
            let dest: *mut InstanceData = self.index_buffer.contents() as *mut InstanceData;
            std::ptr::copy_nonoverlapping(instances.as_ptr(), dest, instances.len());
        }

        self.instance_count = instances.len();
        println!(
            "Instance buffer updated with {} indices",
            self.instance_count
        );
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_buffer(&self, id: BufferId) -> Option<&Buffer> {
        match id.0.get() {
            1 => Some(&self.vertex_buffer),
            2 => Some(&self.index_buffer),
            3 => Some(&self.uniform_buffer),
            4 => Some(&self.instance_buffer),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn get_vertex_count(&self) -> usize {
        self.vertex_count
    }

    #[allow(dead_code)]
    pub fn get_index_count(&self) -> usize {
        self.index_count
    }
}

#[cfg(test)]
mod tests {
    use super::{BufferManager, MAX_INDICES, MAX_VERTICES};
    use crate::renderer::{common::Vertex, RendererError};
    use core::f32;
    use metal::Device;

    // Helper function to compare floats with a small epsilon
    fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    // Helper function to compare Vertex structs
    fn vertex_approx_eq(a: &Vertex, b: &Vertex, epsilon: f32) -> bool {
        approx_eq(a.position[0], b.position[0], epsilon)
            && approx_eq(a.position[1], b.position[1], epsilon)
            && approx_eq(a.position[2], b.position[2], epsilon)
            && approx_eq(a.color[0], b.color[0], epsilon)
            && approx_eq(a.color[1], b.color[1], epsilon)
            && approx_eq(a.color[2], b.color[2], epsilon)
            && approx_eq(a.color[3], b.color[3], epsilon)
    }

    #[test]
    fn test_buffer_manager() {
        println!("IN TEST-BUFFER-MANAGER");
        let device = Device::system_default().unwrap();
        let mut buffer_manager = BufferManager::new(&device).unwrap();

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

        println!("Original vertices: {:?}", vertices);
        println!("Original indices: {:?}", indices);

        assert!(buffer_manager.update_vertex_buffer(&vertices).is_ok());
        assert!(buffer_manager.update_index_buffer(&indices).is_ok());

        assert_eq!(buffer_manager.get_vertex_count(), vertices.len());
        assert_eq!(buffer_manager.get_index_count(), indices.len());

        // Check if the data was correctly written to the buffers
        unsafe {
            let vertex_data = std::slice::from_raw_parts(
                buffer_manager.vertex_buffer.contents() as *const Vertex,
                vertices.len(),
            );
            for (i, vertex) in vertices.iter().enumerate() {
                assert!(
                    vertex_approx_eq(&vertex_data[i], vertex, f32::EPSILON),
                    "Vertex mismatch at index {}: {:?} != {:?}",
                    i,
                    vertex_data[i],
                    vertex
                );
            }

            let index_data = std::slice::from_raw_parts(
                buffer_manager.index_buffer.contents() as *const u32,
                indices.len(),
            );
            println!("Read indices: {:?}", index_data);
            assert_eq!(index_data, indices.as_slice());
        }
    }

    #[test]
    fn test_buffer_overflow() {
        println!("IN TEST-BUFFER-OVERFLOW");
        let device = Device::system_default().unwrap();
        let mut buffer_manager = BufferManager::new(&device).unwrap();

        let too_many_vertices = vec![Vertex::default(); MAX_VERTICES + 1];
        let too_many_indices = vec![0u32; MAX_INDICES + 1];

        assert!(matches!(
            buffer_manager.update_vertex_buffer(&too_many_vertices),
            Err(RendererError::BufferOverflow)
        ));
        assert!(matches!(
            buffer_manager.update_index_buffer(&too_many_indices),
            Err(RendererError::BufferOverflow)
        ));
    }
}
