// dacho/core/renderer/vulkan/backend/src/images/texture.rs

use {
    anyhow::Result,
    ash::vk
};

use {
    super::Image,
    crate::renderer::{buffers::StagingBuffer, commands::CommandPool, devices::{Device, PhysicalDevice}, setup::Instance, VulkanObject}
};


pub struct Texture;

impl Texture {
    pub fn new_image(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        image_data:      &[u8]
    ) -> Result<Image> {
        #[allow(clippy::as_ptr_cast_mut)]
        let data        = image_data.as_ptr() as *mut core::ffi::c_void;
        let buffer_size = core::mem::size_of_val(image_data) as u64;

        let buffer = StagingBuffer::new_buffer(
            instance,
            physical_device,
            device,
            command_pool,
            data,
            buffer_size,
            vk::BufferUsageFlags::TRANSFER_SRC
        )?;

        let (width, height) = {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let y = ((buffer_size / 4 / 2) as f32).sqrt() as u32;

            (y * 2, y)
        };

        let image = Image::new(
            device,
            instance,
            physical_device,
            vk::Extent2D { width, height },
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::SampleCountFlags::TYPE_1
        )?;

        image.transition_layout(
            device, command_pool, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL
        )?;

        buffer.copy_to_image(device, command_pool, &image, width, height)?;

        image.transition_layout(
            device, command_pool, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
        )?;

        buffer.destroy(Some(device));

        Ok(image)
    }
}

