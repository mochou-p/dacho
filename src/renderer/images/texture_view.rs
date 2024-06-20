// dacho/src/renderer/images/texture_view.rs

#![allow(dead_code)]

// crates
use {
    anyhow::Result,
    ash::vk
};

// super
use super::{Image, ImageView};

// crate
use crate::renderer::{
    devices::Device,
    VulkanObject
};

pub struct TextureView;

impl TextureView {
    pub fn new_image_view(
        device:  &Device,
        texture: &Image
    ) -> Result<ImageView> {
        let image_view = ImageView::new(
            device, *texture.raw(), vk::Format::R8G8B8A8_SRGB, vk::ImageAspectFlags::COLOR
        )?;

        Ok(image_view)
    }
}

