// dacho/core/renderer/vulkan/backend/src/buffers/staging.rs

// core
use core::{
    ffi::c_void,
    ptr::copy_nonoverlapping
};

// crates
use {
    anyhow::Result,
    ash::vk
};

// super
use super::Buffer;

// crate
use crate::{
    commands::CommandPool,
    devices::{Device, PhysicalDevice},
    setup::Instance,
    VulkanDrop
};

pub struct StagingBuffer;

impl StagingBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        data:            *mut c_void,
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
            device.raw.map_memory(
                staging_buffer.memory,
                0,
                buffer_size,
                vk::MemoryMapFlags::empty()
            )
        }?;

        unsafe {
            copy_nonoverlapping(data, memory, usize::try_from(buffer_size)?);
            device.raw.unmap_memory(staging_buffer.memory);
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

        staging_buffer.drop(device);

        Ok(buffer)
    }
}

