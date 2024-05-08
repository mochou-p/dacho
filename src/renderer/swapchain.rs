// dacho/src/renderer/swapchain.rs

use {
    anyhow::Result,
    ash::{extensions::khr, vk}
};

use super::{
    device::{Device, PhysicalDevice},
    image::{Image, ImageView},
    instance::Instance,
    render_pass::RenderPass,
    surface::Surface
};

#[cfg(debug_assertions)]
use crate::{
    application::logger::Logger,
    log
};

pub struct Swapchain {
    pub loader:             khr::Swapchain,
    pub raw:                vk::SwapchainKHR,
    pub extent:             vk::Extent2D,
    pub image_count:        usize,
    pub current_image:      usize,
        depth_image:        Image,
        depth_image_view:   ImageView,
        color_image:        Image,
        color_image_view:   ImageView,
        image_views:        Vec<ImageView>,
    pub framebuffers:       Vec<vk::Framebuffer>,
    pub images_available:   Vec<vk::Semaphore>,
    pub images_finished:    Vec<vk::Semaphore>,
    pub may_begin_drawing:  Vec<vk::Fence>
}

impl Swapchain {
    pub fn new(
        instance:        &Instance,
        device:          &Device,
        surface:         &Surface,
        physical_device: &PhysicalDevice,
        render_pass:     &RenderPass,
        width:            u32,
        height:           u32
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating Swapchain");

        let loader = khr::Swapchain::new(&instance.raw, &device.raw);

        let (raw, extent) = {
            let surface_capabilities = unsafe {
                surface.loader.get_physical_device_surface_capabilities(
                    physical_device.raw, surface.raw
                )
            }?;

            let extent = vk::Extent2D::builder()
                .width(width)
                .height(height)
                .build();

            let create_info = vk::SwapchainCreateInfoKHR::builder()
                .surface(surface.raw)
                .min_image_count(surface_capabilities.min_image_count + 1)
                .image_format(vk::Format::B8G8R8A8_SRGB)
                .image_color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
                .image_extent(extent)
                .image_array_layers(1)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .queue_family_indices(&[0])
                .pre_transform(surface_capabilities.current_transform)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(vk::PresentModeKHR::FIFO);
    
            (
                unsafe { loader.create_swapchain(&create_info, None) }?,
                extent
            )
        };

        let images        = unsafe { loader.get_swapchain_images(raw) }?;
        let image_count   = images.len();
        let current_image = 0;

        let mut image_views = Vec::with_capacity(image_count);

        for image in images.iter() {
            let image_view = ImageView::new(
                device,
                image,
                vk::Format::B8G8R8A8_SRGB,
                vk::ImageAspectFlags::COLOR
            )?;

            image_views.push(image_view);
        }

        let depth_image = Image::new(
            device,
            instance,
            physical_device,
            &extent,
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::SampleCountFlags::TYPE_8
        )?;

        let depth_image_view = ImageView::new(
            device,
            &depth_image.raw,
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::ImageAspectFlags::DEPTH
        )?;

        let color_image = Image::new(
            device,
            instance,
            physical_device,
            &extent,
            vk::Format::B8G8R8A8_SRGB,
            vk::ImageUsageFlags::TRANSIENT_ATTACHMENT | vk::ImageUsageFlags::COLOR_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::SampleCountFlags::TYPE_8
        )?;

        let color_image_view = ImageView::new(
            device,
            &color_image.raw,
            vk::Format::B8G8R8A8_SRGB,
            vk::ImageAspectFlags::COLOR
        )?;

        let mut framebuffers = Vec::with_capacity(image_count);

        for image_view in image_views.iter() {
            let attachments = [color_image_view.raw, depth_image_view.raw, image_view.raw];

            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass.raw)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);

            let framebuffer = unsafe { device.raw.create_framebuffer(&create_info, None) }?;

            framebuffers.push(framebuffer);
        }

        let mut images_available = Vec::with_capacity(image_count);
        let mut images_finished  = Vec::with_capacity(image_count);

        {
            let create_info = vk::SemaphoreCreateInfo::builder();

            for _ in 0..image_count {
                let semaphore_available = unsafe { device.raw.create_semaphore(&create_info, None) }?;
                let semaphore_finished  = unsafe { device.raw.create_semaphore(&create_info, None) }?;

                images_available.push(semaphore_available);
                images_finished.push(semaphore_finished);
            }
        }

        let mut may_begin_drawing = Vec::with_capacity(image_count);

        {
            let create_info = vk::FenceCreateInfo::builder()
                .flags(vk::FenceCreateFlags::SIGNALED);

            for _ in 0..image_count {
                let fence = unsafe { device.raw.create_fence(&create_info, None) }?;

                may_begin_drawing.push(fence);
            }
        }

        Ok(
            Self {
                loader,
                raw,
                extent,
                image_count,
                current_image,
                depth_image,
                depth_image_view,
                color_image,
                color_image_view,
                image_views,
                framebuffers,
                images_available,
                images_finished,
                may_begin_drawing
            }
        )
    }

    pub fn destroy(&self, device: &Device) {
        #[cfg(debug_assertions)]
        log!(info, "Destroying Swapchain");

        for fence in self.may_begin_drawing.iter() {
            unsafe { device.raw.destroy_fence(*fence, None); }
        }

        for semaphore in self.images_available.iter() {
            unsafe { device.raw.destroy_semaphore(*semaphore, None); }
        }

        for semaphore in self.images_finished.iter() {
            unsafe { device.raw.destroy_semaphore(*semaphore, None); }
        }

        self.depth_image_view .destroy(device);
        self.depth_image      .destroy(device);
        self.color_image_view .destroy(device);
        self.color_image      .destroy(device);

        for framebuffer in self.framebuffers.iter() {
            unsafe { device.raw.destroy_framebuffer(*framebuffer, None); }
        }

        for image_view in self.image_views.iter() {
            image_view.destroy(device);
        }

        unsafe { self.loader.destroy_swapchain(self.raw, None); }
    }
}

