// dacho/src/renderer/images/image_view.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// crate
use crate::renderer::{
    devices::Device,
    VulkanDrop
};

pub struct ImageView {
    pub raw: vk::ImageView
}

impl ImageView {
    pub fn new(
        device:      &Device,
        image:        vk::Image,
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
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(subresource_range);

        let raw = unsafe { device.raw.create_image_view(&create_info, None) }?;

        Ok(Self { raw })
    }
}

impl VulkanDrop for ImageView {
    fn drop(&self, device: &Device) {
        unsafe { device.raw.destroy_image_view(self.raw, None); }
    }
}

