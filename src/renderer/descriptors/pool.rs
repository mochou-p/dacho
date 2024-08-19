// dacho/src/renderer/descriptors/pool.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// crate
use crate::{
    renderer::{
        devices::Device,
        VulkanObject,
        LOG_SRC
    },
    debug
};

pub struct DescriptorPool {
    raw: vk::DescriptorPool
}

impl DescriptorPool {
    pub fn new(device: &Device) -> Result<Self> {
        debug!(LOG_SRC, "Creating DescriptorPool");

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

            unsafe { device.raw().create_descriptor_pool(&create_info, None) }?
        };

        Ok(Self { raw })
    }
}

impl VulkanObject for DescriptorPool {
    type RawType = vk::DescriptorPool;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn device_destroy(&self, device: &Device) {
        debug!(LOG_SRC, "Destroying DescriptorPool");

        unsafe { device.raw().destroy_descriptor_pool(self.raw, None); }
    }
}

