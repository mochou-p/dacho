// dacho/core/renderer/vulkan/backend/src/buffers/vertex.rs

use core::{ffi::c_void, mem::size_of_val};

use {
    anyhow::Result,
    ash::vk
};

use super::{Buffer, StagingBuffer};
use crate::{
    commands::CommandPool,
    devices::{Device, PhysicalDevice},
    setup::Instance
};


pub struct VertexBuffer;

impl VertexBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        vertices:        &mut [f32]
    ) -> Result<Buffer> {
        let vertex_buffer = {
            let data        = vertices.as_mut_ptr() as *mut c_void;
            let buffer_size = size_of_val(vertices) as u64;
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

