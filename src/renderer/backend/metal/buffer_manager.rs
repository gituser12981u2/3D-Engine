//! Metal buffer management module.
//!
//! This module provides functionality to create and manage Metal buffers for vertex,
//! index, uniform, and instance data, as well as depth textures.

use crate::renderer::{
    common::{Uniforms, Vertex},
    render_queue::InstanceData,
    RendererError,
};
use core_graphics::display::CGSize;
use glam::Mat4;
use log::{debug, trace, warn};
use metal::{
    Buffer, Device, MTLPixelFormat, MTLResourceOptions, MTLStorageMode, MTLTextureUsage, Texture,
    TextureDescriptor,
};

// Constants for maximum buffer size
const MAX_VERTICES: usize = 65_536; // 2^16
const MAX_INDICES: usize = 196_608; // 65536 * 3
const MAX_INSTANCES: usize = 4_096;

/// Manages Metal buffers for vertex, index, uniform, and instance data.
pub struct BufferManager {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub instance_buffer: Buffer,
    pub uniform_buffer: Buffer,
    pub depth_texture: Option<Texture>,
    vertex_count: usize,
    index_count: usize,
    instance_count: usize,
    device: Device,
}

impl BufferManager {
    /// Creates a new `BufferManager` with pre-allocated buffers.
    ///
    /// # Arguments
    ///
    /// * `device` - The Metal device used to create buffers.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `BufferManager` or a `RendererError`.
    pub fn new(device: &Device) -> Result<Self, RendererError> {
        debug!("Creating new BufferManager");
        let vertex_buffer = Self::create_buffer(
            device,
            MAX_VERTICES,
            std::mem::size_of::<Vertex>(),
            "Vertex",
        );
        let index_buffer =
            Self::create_buffer(device, MAX_INDICES, std::mem::size_of::<u32>(), "Index");
        let instance_buffer = Self::create_buffer(
            device,
            MAX_INSTANCES,
            std::mem::size_of::<InstanceData>(),
            "Instance",
        );
        let uniform_buffer = Self::create_buffer(device, 1, std::mem::size_of::<Mat4>(), "Uniform");

        Ok(BufferManager {
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            instance_buffer,
            depth_texture: None,
            vertex_count: 0,
            index_count: 0,
            instance_count: 0,
            device: device.clone(),
        })
    }

    /// Creates a Metal buffer with the specified size and options.
    fn create_buffer(device: &Device, count: usize, stride: usize, name: &str) -> Buffer {
        let buffer = device.new_buffer(
            (count * stride) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeShared,
        );
        buffer.set_label(name);
        debug!("Created {name} buffer: size = {} bytes", count * stride);
        buffer
    }

    /// Generic method to update a buffer with new data.
    fn update_buffer<T: Copy>(
        &self,
        buffer: &Buffer,
        data: &[T],
        max_count: usize,
        buffer_type: &str,
    ) -> Result<usize, RendererError> {
        if data.len() > max_count {
            warn!(
                "{} buffer overflow: {} items exceed maximum of {}",
                buffer_type,
                data.len(),
                max_count
            );
            return Err(RendererError::BufferOverflow);
        }

        unsafe {
            let dest: *mut T = buffer.contents() as *mut T;
            std::ptr::copy_nonoverlapping(data.as_ptr(), dest, data.len());
        }

        trace!("Updated {} buffer with {} items", buffer_type, data.len());
        Ok(data.len())
    }

    /// Updates the vertex buffer with new vertex data.
    ///
    /// # Arguments
    ///
    /// * `vertices` - A slice of vertices to update the buffer with.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `RendererError`.
    pub fn update_vertex_buffer(&mut self, vertices: &[Vertex]) -> Result<(), RendererError> {
        self.vertex_count =
            self.update_buffer(&self.vertex_buffer, vertices, MAX_VERTICES, "vertex")?;
        Ok(())
    }

    /// Updates the index buffer with new index data.
    ///
    /// # Arguments
    ///
    /// * `indices` - A slice of indices to update the buffer with.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `RendererError`.
    pub fn update_index_buffer(&mut self, indices: &[u32]) -> Result<(), RendererError> {
        self.index_count = self.update_buffer(&self.index_buffer, indices, MAX_INDICES, "index")?;
        Ok(())
    }

    /// Updates the instance buffer with new instance data.
    ///
    /// # Arguments
    ///
    /// * `instances` - A slice of instance data to update the buffer with.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success of a `RendererError`.
    pub fn update_instance_buffer(
        &mut self,
        instances: &[InstanceData],
    ) -> Result<(), RendererError> {
        self.instance_count =
            self.update_buffer(&self.instance_buffer, instances, MAX_INSTANCES, "instance")?;
        Ok(())
    }

    /// Updates the uniform buffer with new uniform data.
    ///
    /// # Arguments
    ///
    /// * `uniforms` - A reference to the uniform data to update the buffer with.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or a `RendererError`.
    // TODO: Make a uniform buffer pool to allow multiple objects in a scene
    pub fn update_uniform_buffer(&mut self, uniforms: &Uniforms) -> Result<(), RendererError> {
        trace!("Updating uniform buffer");
        unsafe {
            let dest: *mut Uniforms = self.uniform_buffer.contents() as *mut Uniforms;
            *dest = *uniforms;
        }
        self.uniform_buffer.did_modify_range(metal::NSRange {
            location: 0,
            length: std::mem::size_of::<Uniforms>() as u64,
        });

        Ok(())
    }

    /// Updates the depth texture with a new size.
    ///
    /// # Arguments
    ///
    /// * `size` - The new size for the depth texture.
    pub fn update_depth_texture(&mut self, size: CGSize) {
        let descriptor = TextureDescriptor::new();
        descriptor.set_width(size.width as u64);
        descriptor.set_height(size.height as u64);
        descriptor.set_pixel_format(MTLPixelFormat::Depth32Float);
        descriptor.set_storage_mode(MTLStorageMode::Private);
        descriptor.set_usage(MTLTextureUsage::RenderTarget);

        self.depth_texture = Some(self.device.new_texture(&descriptor));
        trace!("Created depth texture: {}x{}", size.width, size.height);
    }

    /// Ensures that depth texture exists and has the correct size.
    ///
    /// # Arguments
    ///
    /// * `size` -  The required size for the depth texture.
    pub fn ensure_depth_texture(&mut self, size: CGSize) {
        let update_needed = self.depth_texture.as_ref().is_none_or(|texture| {
            texture.width() != size.width as u64 || texture.height() != size.height as u64
        });

        if update_needed {
            self.update_depth_texture(size);
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

    #[allow(dead_code)]
    pub fn get_instance_count(&self) -> usize {
        self.instance_count
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
