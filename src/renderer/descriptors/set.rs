// dacho/src/renderer/descriptors/set.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// super
use super::{pool::*, set_layout::*, uniform::*};

// crate
use crate::renderer::{
    buffers::*,
    devices::logical::*,
    VulkanObject
};

// debug
#[cfg(debug_assertions)]
use crate::{
    application::logger::Logger,
    log
};

pub struct DescriptorSet {
    raw: vk::DescriptorSet
}

impl DescriptorSet {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device:                &Device,
        descriptor_pool:       &DescriptorPool,
        descriptor_set_layout: &DescriptorSetLayout,
        ubo:                   &Buffer
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating DescriptorSet");

        let raw = {
            let set_layouts = [*descriptor_set_layout.raw()];

            let allocate_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(*descriptor_pool.raw())
                .set_layouts(&set_layouts);

            unsafe { device.raw().allocate_descriptor_sets(&allocate_info) }?[0]
        };

        let buffer_infos = [
            vk::DescriptorBufferInfo::builder()
                .buffer(*ubo.raw())
                .offset(0)
                .range(core::mem::size_of::<UniformBufferObject>() as u64)
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

        unsafe { device.raw().update_descriptor_sets(&writes, &[]); }

        Ok(Self { raw })
    }
}

impl VulkanObject for DescriptorSet {
    type RawType = vk::DescriptorSet;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }
}

