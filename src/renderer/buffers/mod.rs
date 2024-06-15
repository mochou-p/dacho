// dacho/src/renderer/buffers/mod.rs

// modules
pub(super) mod index;
pub(super) mod staging;
pub(super) mod vertex;

// crates
use {
    anyhow::Result,
    ash::vk
};

// super
use super::{
    commands::pool::*,
    devices::{logical::*, physical::*},
    images::image::*,
    setup::instance::*,
    VulkanObject
};

// crate
use crate::{
    application::logger::Logger,
    log
};

pub struct Buffer {
        raw:    vk::Buffer,
    pub memory: vk::DeviceMemory
}

impl Buffer {
    pub fn new(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        size:             vk::DeviceSize,
        usage:            vk::BufferUsageFlags,
        properties:       vk::MemoryPropertyFlags
    ) -> Result<Self> {
        let raw = {
            let create_info = vk::BufferCreateInfo::builder()
                .size(size)
                .usage(usage)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);

            unsafe { device.raw().create_buffer(&create_info, None) }?
        };

        let memory = {
            let memory_requirements = unsafe { device.raw().get_buffer_memory_requirements(raw) };
            let memory_properties   = unsafe { instance.raw().get_physical_device_memory_properties(*physical_device.raw()) };

            let memory_type_index = {
                let mut found  = false;
                let mut result = 0;

                for i in 0..memory_properties.memory_type_count {
                    let a = (memory_requirements.memory_type_bits & (1 << i)) != 0;
                    let b = (memory_properties.memory_types[i as usize].property_flags & properties) == properties;

                    if a && b {
                        found  = true;
                        result = i;
                        break;
                    }
                }

                if !found {
                    log!(panic, "Failed to find a suitable memory type");
                }

                result
            };

            let allocate_info = vk::MemoryAllocateInfo::builder()
                .allocation_size(memory_requirements.size)
                .memory_type_index(memory_type_index);

            unsafe { device.raw().allocate_memory(&allocate_info, None) }?
        };

        unsafe { device.raw().bind_buffer_memory(raw, memory, 0) }?;

        Ok(Self { raw, memory })
    }

    pub fn copy(
        device:       &Device,
        command_pool: &CommandPool,
        src_buffer:   &Self,
        dst_buffer:   &Self,
        size:          vk::DeviceSize
    ) -> Result<()> {
        let command_buffer = command_pool.begin_single_time_commands(device)?;

        {
            let copy_region = vk::BufferCopy::builder().size(size);

            unsafe { device.raw().cmd_copy_buffer(command_buffer, src_buffer.raw, dst_buffer.raw, &[*copy_region]); }
        }

        command_pool.end_single_time_commands(device, &command_buffer)?;

        Ok(())
    }

    pub fn copy_to_image(
        &self,
        device:       &Device,
        command_pool: &CommandPool,
        image:        &Image,
        width:         u32,
        height:        u32
    ) -> Result<()> {
        let command_buffer = command_pool.begin_single_time_commands(device)?;

        let subresource_range = vk::ImageSubresourceLayers::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let region = vk::BufferImageCopy::builder()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(subresource_range)
            .image_offset(vk::Offset3D { x: 0,  y: 0,   z:     0 })
            .image_extent(vk::Extent3D { width, height, depth: 1 });

        unsafe {
            device.raw().cmd_copy_buffer_to_image(
                command_buffer,
                self.raw,
                *image.raw(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[*region]
            );
        }

        command_pool.end_single_time_commands(device, &command_buffer)?;

        Ok(())
    }
}

impl VulkanObject for Buffer {
    type RawType = vk::Buffer;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn destroy(&self, device: Option<&Device>) {
        if let Some(device) = device {
            unsafe {
                device.raw().destroy_buffer(self.raw, None);
                device.raw().free_memory(self.memory, None);
            }
        } else {
            log!(panic, "Expected Option<&Device>, got None");
        }
    }
}

