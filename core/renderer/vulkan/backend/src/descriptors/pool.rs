// dacho/core/renderer/vulkan/backend/src/descriptors/pool.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// crate
use crate::{
    devices::Device,
    VulkanDrop
};

use dacho_log::{create_log, destroy_log};

pub struct DescriptorPool {
    pub raw: vk::DescriptorPool
}

impl DescriptorPool {
    pub fn new(device: &Device) -> Result<Self> {
        create_log!(debug);

        let raw = {
            let pool_sizes = [
                vk::DescriptorPoolSize::builder()
                    .ty(vk::DescriptorType::UNIFORM_BUFFER)
                    .descriptor_count(1)
                    .build()
            ];

            let create_info = vk::DescriptorPoolCreateInfo::builder()
                .pool_sizes(&pool_sizes)
                .max_sets(1);

            unsafe { device.raw.create_descriptor_pool(&create_info, None) }?
        };

        Ok(Self { raw })
    }
}

impl VulkanDrop for DescriptorPool {
    fn drop(&self, device: &Device) {
        destroy_log!(debug);

        unsafe { device.raw.destroy_descriptor_pool(self.raw, None); }
    }
}

