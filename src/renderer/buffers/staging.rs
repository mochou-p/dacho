// dacho/src/renderer/buffers/staging.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// super
use super::*;

// crate
use crate::renderer::{
    commands::pool::*,
    devices::{logical::*, physical::*},
    setup::instance::*,
    VulkanObject
};

pub struct StagingBuffer;

impl StagingBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        data:            *mut core::ffi::c_void,
        buffer_size:      u64,
        buffer_type:      vk::BufferUsageFlags
    ) -> Result<Buffer> {
        let staging_buffer = {
            let usage      = vk::BufferUsageFlags::TRANSFER_SRC;
            let properties = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;

            Buffer::new(
                instance,
                physical_device,
                device,
                buffer_size,
                usage,
                properties
            )?
        };

        let memory = unsafe {
            device.raw().map_memory(
                staging_buffer.memory,
                0,
                buffer_size,
                vk::MemoryMapFlags::empty()
            )
        }?;

        unsafe {
            #[allow(unused_unsafe)] // extra unsafe to compile trough a clippy false positive
            core::ptr::copy_nonoverlapping(unsafe { data }, memory, buffer_size as usize);
            device.raw().unmap_memory(staging_buffer.memory);
        }

        let buffer = {
            let usage      = vk::BufferUsageFlags::TRANSFER_DST | buffer_type;
            let properties = vk::MemoryPropertyFlags::DEVICE_LOCAL;

            Buffer::new(
                instance,
                physical_device,
                device,
                buffer_size,
                usage,
                properties
            )?
        };

        Buffer::copy(
            device,
            command_pool,
            &staging_buffer,
            &buffer,
            buffer_size
        )?;

        staging_buffer.destroy(Some(device));

        Ok(buffer)
    }
}

