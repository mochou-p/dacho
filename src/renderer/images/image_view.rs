// dacho/src/renderer/images/image_view.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// crate
use crate::{
    app::logger::Logger,
    renderer::{
        devices::Device,
        VulkanObject
    },
    log
};

pub struct ImageView {
    raw: vk::ImageView
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

        let raw = unsafe { device.raw().create_image_view(&create_info, None) }?;

        Ok(Self { raw })
    }
}

impl VulkanObject for ImageView {
    type RawType = vk::ImageView;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn destroy(&self, device: Option<&Device>) {
        if let Some(device) = device {
            unsafe { device.raw().destroy_image_view(self.raw, None); }
        } else {
            log!(panic, "Expected Option<&Device>, got None");
        }
    }
}

