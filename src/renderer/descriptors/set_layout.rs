// dacho/renderer/descriptors/set_layout.rs

use {
    anyhow::Result,
    ash::vk
};

use crate::{
    application::logger::Logger,
    renderer::{
        devices::logical::*,
        VulkanObject
    },
    log
};

pub struct DescriptorSetLayout {
    raw: vk::DescriptorSetLayout
}

impl DescriptorSetLayout {
    pub fn new(device: &Device) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating DescriptorSetLayout");

        let raw = {
            let ubo_bindings = [
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(0)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .stage_flags(vk::ShaderStageFlags::VERTEX)
                    .build(),
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(1)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::SAMPLER)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                    .build(),
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(2)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                    .build()
            ];

            let create_info = vk::DescriptorSetLayoutCreateInfo::builder()
                .bindings(&ubo_bindings);

            unsafe { device.raw().create_descriptor_set_layout(&create_info, None) }?
        };

        Ok(Self { raw })
    }
}

impl VulkanObject for DescriptorSetLayout {
    type RawType = vk::DescriptorSetLayout;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn destroy(&self, device: Option<&Device>) {
        #[cfg(debug_assertions)]
        log!(info, "Destroying DescriptorSetLayout");

        if let Some(device) = device {
            unsafe { device.raw().destroy_descriptor_set_layout(self.raw, None); }
        } else {
            log!(panic, "Expected Option<&Device>, got None");
        }
    }
}

