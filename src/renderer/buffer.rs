// dacho/src/renderer/buffer.rs

use anyhow::Result;

use ash::vk;

pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory
}

impl Buffer {
    pub fn new(
        instance:        &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        device:          &ash::Device,
        size:             vk::DeviceSize,
        usage:            vk::BufferUsageFlags,
        properties:       vk::MemoryPropertyFlags
    ) -> Result<Self> {
        let buffer = {
            let create_info = vk::BufferCreateInfo::builder()
                .size(size)
                .usage(usage)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);

            unsafe { device.create_buffer(&create_info, None) }?
        };

        let memory = {
            let memory_requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
            let memory_properties   = unsafe { instance.get_physical_device_memory_properties(*physical_device) };

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
                    panic!("Failed to find a suitable memory type");
                }

                result
            };

            let allocate_info = vk::MemoryAllocateInfo::builder()
                .allocation_size(memory_requirements.size)
                .memory_type_index(memory_type_index);

            unsafe { device.allocate_memory(&allocate_info, None) }?
        };

        unsafe { device.bind_buffer_memory(buffer, memory, 0) }?;

        Ok(
            Self {
                buffer,
                memory
            }
        )
    }

    pub fn copy(
        device:       &ash::Device,
        queue:        &vk::Queue,
        command_pool: &vk::CommandPool,
        src_buffer:   &vk::Buffer,
        dst_buffer:   &vk::Buffer,
        size:          vk::DeviceSize
    ) -> Result<()> {
        let command_buffers = {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_pool(*command_pool)
                .command_buffer_count(1);

            unsafe { device.allocate_command_buffers(&allocate_info) }?
        };

        {
            let begin_info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

            unsafe { device.begin_command_buffer(command_buffers[0], &begin_info) }?;
        }

        {
            let copy_regions = [
                vk::BufferCopy::builder()
                    .size(size)
                    .build()
            ];

            unsafe { device.cmd_copy_buffer(command_buffers[0], *src_buffer, *dst_buffer, &copy_regions); }
        }

        unsafe { device.end_command_buffer(command_buffers[0]) }?;

        {
            let submit_infos = [
                vk::SubmitInfo::builder()
                    .command_buffers(&command_buffers)
                    .build()
            ];

            unsafe { device.queue_submit(*queue, &submit_infos, vk::Fence::null()) }?;
        }

        unsafe { device.queue_wait_idle(*queue) }?;
        unsafe { device.free_command_buffers(*command_pool, &command_buffers); }

        Ok(())
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.buffer, None);
            device.free_memory(self.memory, None);
        }
    }
}

struct SomeBuffer;

impl SomeBuffer {
    pub fn new(
        instance:        &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        device:          &ash::Device,
        queue:           &vk::Queue,
        command_pool:    &vk::CommandPool,
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
            device.map_memory(
                staging_buffer.memory,
                0,
                buffer_size,
                vk::MemoryMapFlags::empty()
            )
        }?;

        unsafe {
            std::ptr::copy_nonoverlapping(data, memory, buffer_size as usize);
            device.unmap_memory(staging_buffer.memory);
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
            queue,
            command_pool,
            &staging_buffer.buffer,
            &buffer.buffer,
            buffer_size
        )?;

        staging_buffer.destroy(device);

        Ok(buffer)
    }
}

pub struct VertexBuffer;

impl VertexBuffer {
    pub fn new(
        instance:        &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        device:          &ash::Device,
        queue:           &vk::Queue,
        command_pool:    &vk::CommandPool,
        vertices:        &Vec<f32>,
    ) -> Result<Buffer> {
        let vertex_buffer = {
            let data        = vertices.as_ptr() as *mut std::ffi::c_void;
            let buffer_size = (std::mem::size_of::<f32>() * vertices.len()) as u64;
            let buffer_type = vk::BufferUsageFlags::VERTEX_BUFFER;

            SomeBuffer::new(
                instance,
                physical_device,
                device,
                queue,
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
    pub fn new(
        instance:        &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        device:          &ash::Device,
        queue:           &vk::Queue,
        command_pool:    &vk::CommandPool,
        indices:         &Vec<u16>
    ) -> Result<Buffer> {
        let index_buffer = {
            let data        = indices.as_ptr() as *mut std::ffi::c_void;
            let buffer_size = (std::mem::size_of::<u16>() * indices.len()) as u64;
            let buffer_type = vk::BufferUsageFlags::INDEX_BUFFER;

            SomeBuffer::new(
                instance,
                physical_device,
                device,
                queue,
                command_pool,
                data,
                buffer_size,
                buffer_type
            )?
        };

        Ok(index_buffer)
    }
}

