// dacho/src/renderer/command.rs

use anyhow::Result;

use ash::vk;

use super::swapchain::Swapchain;

pub struct CommandBuffers {
    pub command_buffers: Vec<vk::CommandBuffer>
}

impl CommandBuffers {
    pub fn new(
       command_pool: &vk::CommandPool,
       swapchain:    &Swapchain,
       device:       &ash::Device
    ) -> Result<Self> {
        let command_buffers = {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(*command_pool)
                .command_buffer_count(swapchain.image_count as u32);

            unsafe { device.allocate_command_buffers(&allocate_info) }?
        };

        Ok(
            Self {
                command_buffers
            }
        )
    }
}

