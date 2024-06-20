// dacho/src/renderer/descriptors/pool.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// crate
use crate::{
    application::logger::Logger,
    renderer::{
        devices::Device,
        VulkanObject
    },
    log
};

pub struct DescriptorPool {
    raw: vk::DescriptorPool
}

impl DescriptorPool {
    pub fn new(device: &Device) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating DescriptorPool");

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

    fn destroy(&self, device: Option<&Device>) {
        #[cfg(debug_assertions)]
        log!(info, "Destroying DescriptorPool");

        if let Some(device) = device {
            unsafe { device.raw().destroy_descriptor_pool(self.raw, None); }
        } else {
            log!(panic, "Expected Option<&Device>, got None");
        }
    }
}

