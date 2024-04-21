// dacho/src/renderer/swapchain.rs

use anyhow::Result;

use ash::{extensions::khr, vk};

use super::{
    device::{Device, PhysicalDevice},
    instance::Instance,
    render_pass::RenderPass,
    surface::Surface
};

pub struct Swapchain {
    pub loader:             khr::Swapchain,
    pub raw:                vk::SwapchainKHR,
    pub extent:             vk::Extent2D,
    pub image_count:        usize,
    pub current_image:      usize,
        depth_image:        vk::Image,
        depth_image_view:   vk::ImageView,
        depth_image_memory: vk::DeviceMemory,
        color_image:        vk::Image,
        color_image_view:   vk::ImageView,
        color_image_memory: vk::DeviceMemory,
        image_views:        Vec<vk::ImageView>,
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
        let loader = khr::Swapchain::new(&instance.raw, &device.raw);

        let (raw, extent) = {
            let surface_capabilities = unsafe {
                surface.loader.get_physical_device_surface_capabilities(
                    physical_device.raw, surface.raw
                )
            }?;

            let queue_families = [0];

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
                .queue_family_indices(&queue_families)
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
            let image_view = Self::create_image_view(
                device,
                image,
                vk::Format::B8G8R8A8_SRGB,
                vk::ImageAspectFlags::COLOR
            )?;

            image_views.push(image_view);
        }

        let (depth_image, depth_image_memory) = Self::create_image(
            device,
            instance,
            physical_device,
            &extent,
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL
        )?;

        let depth_image_view = Self::create_image_view(
            device,
            &depth_image,
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::ImageAspectFlags::DEPTH
        )?;

        let (color_image, color_image_memory) = Self::create_image(
            device,
            instance,
            physical_device,
            &extent,
            vk::Format::B8G8R8A8_SRGB,
            vk::ImageUsageFlags::TRANSIENT_ATTACHMENT | vk::ImageUsageFlags::COLOR_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL
        )?;

        let color_image_view = Self::create_image_view(
            device,
            &color_image,
            vk::Format::B8G8R8A8_SRGB,
            vk::ImageAspectFlags::COLOR
        )?;

        let mut framebuffers = Vec::with_capacity(image_count);

        for image_view in image_views.iter() {
            let attachments = [color_image_view, depth_image_view, *image_view];

            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass.raw)
                .attachments(&attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);

            let framebuffer = unsafe { device.raw.create_framebuffer(&create_info, None) }?;

            framebuffers.push(framebuffer);
        }

        let mut images_available = vec![];
        let mut images_finished  = vec![];

        {
            let create_info = vk::SemaphoreCreateInfo::builder();

            for _ in 0..image_count {
                let semaphore_available = unsafe { device.raw.create_semaphore(&create_info, None) }?;
                let semaphore_finished  = unsafe { device.raw.create_semaphore(&create_info, None) }?;

                images_available.push(semaphore_available);
                images_finished.push(semaphore_finished);
            }
        }

        let mut may_begin_drawing = vec![];

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
                depth_image_memory,
                depth_image_view,
                color_image,
                color_image_memory,
                color_image_view,
                image_views,
                framebuffers,
                images_available,
                images_finished,
                may_begin_drawing
            }
        )
    }

    fn create_image(
        device:          &Device,
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        extent_2d:       &vk::Extent2D,
        format:           vk::Format,
        usage:            vk::ImageUsageFlags,
        properties:       vk::MemoryPropertyFlags
    ) -> Result<(vk::Image, vk::DeviceMemory)> {
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

        let image = unsafe { device.raw.create_image(&create_info, None) }?;

        let memory_requirements = unsafe { device.raw.get_image_memory_requirements(image) };
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

        let image_memory = unsafe { device.raw.allocate_memory(&allocate_info, None) }?;

        unsafe { device.raw.bind_image_memory(image, image_memory, 0) }?;

        Ok((image, image_memory))
    }

    fn create_image_view(
        device:      &Device,
        image:       &vk::Image,
        format:       vk::Format,
        aspect_mask:  vk::ImageAspectFlags
    ) -> Result<vk::ImageView> {
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

        let image_view = unsafe { device.raw.create_image_view(&create_info, None) }?;

        Ok(image_view)
    }

    pub fn destroy(&self, device: &Device) {
        for fence in self.may_begin_drawing.iter() {
            unsafe { device.raw.destroy_fence(*fence, None); }
        }

        for semaphore in self.images_available.iter() {
            unsafe { device.raw.destroy_semaphore(*semaphore, None); }
        }

        for semaphore in self.images_finished.iter() {
            unsafe { device.raw.destroy_semaphore(*semaphore, None); }
        }

        unsafe {
            device.raw.destroy_image_view(self.depth_image_view, None);
            device.raw.destroy_image(self.depth_image, None);
            device.raw.free_memory(self.depth_image_memory, None);
            device.raw.destroy_image_view(self.color_image_view, None);
            device.raw.destroy_image(self.color_image, None);
            device.raw.free_memory(self.color_image_memory, None);
        }

        for framebuffer in self.framebuffers.iter() {
            unsafe { device.raw.destroy_framebuffer(*framebuffer, None); }
        }

        for image_view in self.image_views.iter() {
            unsafe { device.raw.destroy_image_view(*image_view, None); }
        }

        unsafe { self.loader.destroy_swapchain(self.raw, None); }
    }
}

