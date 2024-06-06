// dacho/src/renderer/descriptors/set.rs

use {
    anyhow::Result,
    ash::vk
};

use {
    super::{pool::*, set_layout::*, uniform::*},
    crate::renderer::{
        buffers::*,
        devices::logical::*,
        images::{image_view::*, sampler::*},
        VulkanObject
    }
};

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
        ubo:                   &Buffer,
        sampler:               &Sampler,
        image_view:            &ImageView
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
                .range(std::mem::size_of::<UniformBufferObject>() as u64)
                .build()
        ];

        let sampler_infos = [
            vk::DescriptorImageInfo::builder()
                .image_view(vk::ImageView::null())
                .sampler(*sampler.raw())
                .build()
        ];

        let image_view_infos = [
            vk::DescriptorImageInfo::builder()
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image_view(*image_view.raw())
                .sampler(vk::Sampler::null())
                .build()
        ];

        let writes = [
            vk::WriteDescriptorSet::builder()
                .dst_set(raw)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&buffer_infos)
                .build(),
            vk::WriteDescriptorSet::builder()
                .dst_set(raw)
                .dst_binding(1)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::SAMPLER)
                .image_info(&sampler_infos)
                .build(),
            vk::WriteDescriptorSet::builder()
                .dst_set(raw)
                .dst_binding(2)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&image_view_infos)
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

