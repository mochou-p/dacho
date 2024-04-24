// dacho/src/renderer/image.rs

use anyhow::Result;

use ash::vk;

use super::{
    device::{Device, PhysicalDevice},
    instance::Instance
};

pub struct Image {
    pub raw:    vk::Image,
        memory: vk::DeviceMemory
}

impl Image {
    pub fn new(
        device:          &Device,
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        extent_2d:       &vk::Extent2D,
        format:           vk::Format,
        usage:            vk::ImageUsageFlags,
        properties:       vk::MemoryPropertyFlags
    ) -> Result<Self> {
        let extent = vk::Extent3D::builder()
            .width(extent_2d.width)
            .height(extent_2d.height)
            .depth(1)
            .build();

        let create_info = vk::ImageCreateInfo::builder()
            .extent(extent)
            .format(format)
            .usage(usage)
            .image_type(vk::ImageType::TYPE_2D)
            .mip_levels(1)
            .array_layers(1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .samples(vk::SampleCountFlags::TYPE_8)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let _ = vk::ImageCreateInfo::builder();

        let raw = unsafe { device.raw.create_image(&create_info, None) }?;

        let memory_requirements = unsafe { device.raw.get_image_memory_requirements(raw) };
        let memory_properties   = unsafe { instance.raw.get_physical_device_memory_properties(physical_device.raw) };

        let memory_type_index = {
            let mut found  = false;
            let mut result = 0;

            for i in 0..memory_properties.memory_type_count {
                let a = (memory_requirements.memory_type_bits & (1 << i)) != 0;
                let b = (memory_properties.memory_types[i as usize].property_flags & properties) == properties;

                if a && b {
                    found  = true;
                    result = i;
                    break;
                }
            }

            if !found {
                panic!("Failed to find a suitable memory type");
            }

            result
        };

        let allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.raw.allocate_memory(&allocate_info, None) }?;

        unsafe { device.raw.bind_image_memory(raw, memory, 0) }?;

        Ok(Self { raw, memory })
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.raw.destroy_image(self.raw, None);
            device.raw.free_memory(self.memory, None);
        }
    }
}

pub struct ImageView {
    pub raw: vk::ImageView
}

impl ImageView {
    pub fn new(
        device:      &Device,
        image:       &vk::Image,
        format:       vk::Format,
        aspect_mask:  vk::ImageAspectFlags
    ) -> Result<Self> {
        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspect_mask)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let create_info = vk::ImageViewCreateInfo::builder()
            .image(*image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(subresource_range);

        let raw = unsafe { device.raw.create_image_view(&create_info, None) }?;

        Ok(Self { raw })
    }

    pub fn destroy(&self, device: &Device) {
        unsafe { device.raw.destroy_image_view(self.raw, None); }
    }
}

