// dacho/src/renderer/command.rs

use anyhow::Result;

use ash::vk;

use super::swapchain::Swapchain;

pub struct CommandPool {
    pub command_pool: vk::CommandPool
}

impl CommandPool {
    pub fn new(device: &ash::Device) -> Result<Self> {
        let command_pool = {
            let create_info = vk::CommandPoolCreateInfo::builder()
                .queue_family_index(0)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

            unsafe { device.create_command_pool(&create_info, None) }?
        };

        Ok(
            Self {
                command_pool
            }
        )
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}

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

