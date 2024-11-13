// dacho/core/renderer/vulkan/backend/src/commands/pool.rs

use {
    anyhow::Result,
    ash::vk
};

use crate::{devices::Device, VulkanDrop};

use dacho_log::{create_log, destroy_log};


pub struct CommandPool {
    pub raw: vk::CommandPool
}

impl CommandPool {
    pub fn new(device: &Device) -> Result<Self> {
        create_log!(debug);

        let raw = {
            let create_info = vk::CommandPoolCreateInfo::builder()
                .queue_family_index(0)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

            unsafe { device.raw.create_command_pool(&create_info, None) }?
        };

        Ok(Self { raw })
    }

    pub fn begin_single_time_commands(&self, device: &Device) -> Result<vk::CommandBuffer> {
        let command_buffer = {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_pool(self.raw)
                .command_buffer_count(1);

            unsafe { device.raw.allocate_command_buffers(&allocate_info) }?[0]
        };

        {
            let begin_info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

            unsafe { device.raw.begin_command_buffer(command_buffer, &begin_info) }?;
        }

        Ok(command_buffer)
    }

    pub fn end_single_time_commands(
        &self,
        device:         &Device,
        command_buffer:  vk::CommandBuffer
    ) -> Result<()> {
        unsafe { device.raw.end_command_buffer(command_buffer) }?;

        let command_buffers = [command_buffer];

        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(&command_buffers);

        unsafe { device.raw.queue_submit(device.queue, &[*submit_info], vk::Fence::null()) }?;

        unsafe { device.raw.queue_wait_idle(device.queue) }?;
        unsafe { device.raw.free_command_buffers(self.raw, &command_buffers); }

        Ok(())
    }
}

impl VulkanDrop for CommandPool {
    fn drop(&self, device: &Device) {
        destroy_log!(debug);

        unsafe { device.raw.destroy_command_pool(self.raw, None); }
    }
}

