// dacho/src/renderer/descriptor.rs

use anyhow::Result;

use ash::vk;

pub struct DescriptorSetLayout {
    pub descriptor_set_layout: vk::DescriptorSetLayout
}

impl DescriptorSetLayout {
    pub fn new(device: &ash::Device) -> Result<Self> {
        let descriptor_set_layout = {
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

            unsafe { device.create_descriptor_set_layout(&create_info, None) }?
        };

        Ok(Self { descriptor_set_layout })
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe { device.destroy_descriptor_set_layout(self.descriptor_set_layout, None); }
    }
}

