// dacho/src/renderer/buffers/vertex.rs

use {
    anyhow::Result,
    ash::vk
};

use {
    super::{staging::*, *},
    crate::renderer::{
        commands::pool::*,
        devices::{logical::*, physical::*},
        setup::instance::*
    }
};

pub struct VertexBuffer;

impl VertexBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        vertices:        &[f32]
    ) -> Result<Buffer> {
        let vertex_buffer = {
            let data        = vertices.as_ptr() as *mut std::ffi::c_void;
            let buffer_size = std::mem::size_of_val(vertices) as u64;
            let buffer_type = vk::BufferUsageFlags::VERTEX_BUFFER;

            StagingBuffer::new_buffer(
                instance,
                physical_device,
                device,
                command_pool,
                data,
                buffer_size,
                buffer_type
            )?
        };

        Ok(vertex_buffer)
    }
}

