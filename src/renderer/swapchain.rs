// dacho/src/renderer/swapchain.rs

use anyhow::Result;

use ash::{
    extensions::khr,
    vk
};

use super::surface::Surface;

pub struct Swapchain {
    pub loader:            khr::Swapchain,
    pub swapchain:         vk::SwapchainKHR,
    pub extent:            vk::Extent2D,
    pub image_count:       usize,
    pub current_image:     usize,
        image_views:       Vec<vk::ImageView>,
    pub framebuffers:      Vec<vk::Framebuffer>,
    pub images_available:  Vec<vk::Semaphore>,
    pub images_finished:   Vec<vk::Semaphore>,
    pub may_begin_drawing: Vec<vk::Fence>
}

impl Swapchain {
    pub fn new(
        instance:        &ash::Instance,
        device:          &ash::Device,
        surface:         &Surface,
        physical_device: &vk::PhysicalDevice,
        render_pass:     &vk::RenderPass,
        width:            u32,
        height:           u32
    ) -> Result<Self> {
        let loader = khr::Swapchain::new(&instance, &device);

        let (swapchain, extent) = {
            let surface_capabilities = unsafe {
                surface.loader.get_physical_device_surface_capabilities(
                    *physical_device, surface.surface
                )
            }?;

            let queue_families = [0];

            let extent = vk::Extent2D::builder()
                .width(width)
                .height(height)
                .build();

            let create_info = vk::SwapchainCreateInfoKHR::builder()
                .surface(surface.surface)
                .min_image_count(surface_capabilities.min_image_count + 1)
                .image_format(vk::Format::R5G6B5_UNORM_PACK16)
                .image_color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
                .image_extent(extent)
                .image_array_layers(1)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .queue_family_indices(&queue_families)
                .pre_transform(surface_capabilities.current_transform)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(vk::PresentModeKHR::FIFO);
    
            (
                unsafe { loader.create_swapchain(&create_info, None) }?,
                extent
            )
        };

        let images        = unsafe { loader.get_swapchain_images(swapchain) }?;
        let image_count   = images.len();
        let current_image = 0;

        let mut image_views = Vec::with_capacity(image_count);

        for image in &images {
            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);

            let create_info = vk::ImageViewCreateInfo::builder()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(vk::Format::R5G6B5_UNORM_PACK16)
                .subresource_range(*subresource_range);

            let image_view = unsafe { device.create_image_view(&create_info, None) }?;

            image_views.push(image_view);
        }

        let mut framebuffers = Vec::with_capacity(image_count);

        for image_view in &image_views {
            let attachments = [*image_view];

            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(*render_pass)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);

            let framebuffer = unsafe { device.create_framebuffer(&create_info, None) }?;

            framebuffers.push(framebuffer);
        }

        let mut images_available = vec![];
        let mut images_finished  = vec![];

        {
            let create_info = vk::SemaphoreCreateInfo::builder();

            for _ in 0..image_count {
                let semaphore_available = unsafe { device.create_semaphore(&create_info, None) }?;
                let semaphore_finished  = unsafe { device.create_semaphore(&create_info, None) }?;

                images_available.push(semaphore_available);
                images_finished.push(semaphore_finished);
            }
        }

        let mut may_begin_drawing = vec![];

        {
            let create_info = vk::FenceCreateInfo::builder()
                .flags(vk::FenceCreateFlags::SIGNALED);

            for _ in 0..image_count {
                let fence = unsafe { device.create_fence(&create_info, None) }?;

                may_begin_drawing.push(fence);
            }
        }

        Ok(
            Self {
                loader,
                swapchain,
                extent,
                image_count,
                current_image,
                image_views,
                framebuffers,
                images_available,
                images_finished,
                may_begin_drawing
            }
        )
    }

    pub fn destroy(&self, device: &ash::Device) {
        for fence in &self.may_begin_drawing {
            unsafe { device.destroy_fence(*fence, None); }
        }

        for semaphore in &self.images_available {
            unsafe { device.destroy_semaphore(*semaphore, None); }
        }

        for semaphore in &self.images_finished {
            unsafe { device.destroy_semaphore(*semaphore, None); }
        }

        for framebuffer in &self.framebuffers {
            unsafe { device.destroy_framebuffer(*framebuffer, None); }
        }

        for image_view in &self.image_views {
            unsafe { device.destroy_image_view(*image_view, None); }
        }

        unsafe {
            self.loader.destroy_swapchain(self.swapchain, None);
        }
    }
}

