// dacho/src/renderer/descriptors/set_layout.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// crate
use crate::{
    renderer::{
        devices::Device,
        VulkanObject
    },
    create_log, destroy_log
};

pub struct DescriptorSetLayout {
    raw: vk::DescriptorSetLayout
}

impl DescriptorSetLayout {
    pub fn new(device: &Device) -> Result<Self> {
        create_log!(debug);

        let raw = {
            let ubo_bindings = [
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(0)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .stage_flags(vk::ShaderStageFlags::VERTEX)
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

    fn device_destroy(&self, device: &Device) {
        destroy_log!(debug);

        unsafe { device.raw().destroy_descriptor_set_layout(self.raw, None); }
    }
}

