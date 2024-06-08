// dacho/src/renderer/buffers/index.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// super
use super::{staging::*, *};

// crate
use crate::renderer::{
    commands::pool::*,
    devices::{logical::*, physical::*},
    setup::instance::*
};

pub struct IndexBuffer;

impl IndexBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        indices:         &[u32]
    ) -> Result<Buffer> {
        let index_buffer = {
            let data        = indices.as_ptr() as *mut std::ffi::c_void;
            let buffer_size = std::mem::size_of_val(indices) as u64;
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

