// dacho/src/renderer/buffer.rs

use {
    anyhow::Result,
    ash::vk
};

use {
    super::{
        command::CommandPool,
        device::{Device, PhysicalDevice},
        image::Image,
        instance::Instance,
        VulkanObject
    },
    crate::{
        application::logger::Logger,
        log
    }
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
        src_buffer:   &Buffer,
        dst_buffer:   &Buffer,
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

pub struct StagingBuffer;

impl StagingBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        data:            *mut std::ffi::c_void,
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
            std::ptr::copy_nonoverlapping(data, memory, buffer_size as usize);
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

pub struct VertexBuffer;

impl VertexBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        vertices:        &[f32],
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

