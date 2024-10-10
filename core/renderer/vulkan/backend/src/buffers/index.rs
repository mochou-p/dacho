// dacho/core/renderer/vulkan/backend/src/buffers/index.rs

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


pub struct IndexBuffer;

impl IndexBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        indices:         &mut [u32]
    ) -> Result<Buffer> {
        let index_buffer = {
            let data        = indices.as_mut_ptr() as *mut c_void;
            let buffer_size = size_of_val(indices) as u64;
            let buffer_type = vk::BufferUsageFlags::INDEX_BUFFER;

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

        Ok(index_buffer)
    }
}

