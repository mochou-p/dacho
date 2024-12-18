// dacho/core/renderer/vulkan/backend/src/descriptors/set.rs

use core::mem::size_of;

use {
    anyhow::Result,
    ash::vk
};

use {
    super::{DescriptorPool, DescriptorSetLayout, UniformBufferObject},
    crate::{buffers::Buffer, devices::Device}
};

use dacho_log::create_log;


pub struct DescriptorSet {
    pub raw: vk::DescriptorSet
}

impl DescriptorSet {
    pub fn new(
        device:                &Device,
        descriptor_pool:       &DescriptorPool,
        descriptor_set_layout: &DescriptorSetLayout,
        ubo:                   &Buffer
    ) -> Result<Self> {
        create_log!(debug);

        let raw = {
            let set_layouts = [descriptor_set_layout.raw];

            let allocate_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(descriptor_pool.raw)
                .set_layouts(&set_layouts);

            unsafe { device.raw.allocate_descriptor_sets(&allocate_info) }?[0]
        };

        let buffer_infos = [
            vk::DescriptorBufferInfo::builder()
                .buffer(ubo.raw)
                .offset(0)
                .range(size_of::<UniformBufferObject>() as u64)
                .build()
        ];

        let writes = [
            vk::WriteDescriptorSet::builder()
                .dst_set(raw)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&buffer_infos)
                .build()
        ];

        unsafe { device.raw.update_descriptor_sets(&writes, &[]); }

        Ok(Self { raw })
    }
}

