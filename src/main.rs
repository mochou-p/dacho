// dacho/src/main.rs

use anyhow::{Context, Result};

use ash::{
    extensions::khr::{Surface, Swapchain},
    vk
};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder
};

fn read_shader<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<u32>> {
    let u8_vec = std::fs::read(path)?;

    Ok(unsafe { std::mem::transmute(u8_vec) })
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let entry = unsafe { ash::Entry::load() }?;

    let instance = {
        let application_info = vk::ApplicationInfo::builder()
            .api_version(vk::API_VERSION_1_3);

        let extension_names = ash_window::enumerate_required_extensions(
            event_loop.raw_display_handle()
        )?;

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(extension_names);

        unsafe { entry.create_instance(&create_info, None) }?
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }?
        .into_iter()
        .next()
        .context("No physical devices")?;

    let device = {
        let queue_priorities = [1.0];

        let extension_names = [Swapchain::name().as_ptr()];

        let queue_create_infos = [
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(0)
                .queue_priorities(&queue_priorities)
                .build()
        ];

        let create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(&extension_names);

        unsafe { instance.create_device(physical_device, &create_info, None) }?
    };

    let _queue = unsafe { device.get_device_queue(0, 0) };

    let window = WindowBuilder::new()
        .with_title("dacho")
        .build(&event_loop)?;

    let surface_loader = Surface::new(&entry, &instance);
    
    let surface = unsafe { ash_window::create_surface(
        &entry,
        &instance,
        window.raw_display_handle(),
        window.raw_window_handle(),
        None
    ) }?;

    let swapchain_loader = Swapchain::new(&instance, &device);

    let swapchain = {
        let surface_capabilities = unsafe { surface_loader.get_physical_device_surface_capabilities(
            physical_device, surface
        ) }?;

        let surface_formats = unsafe { surface_loader.get_physical_device_surface_formats(
            physical_device, surface
        ) }?;

        let queue_families = [0];

        let create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(
                3.max(surface_capabilities.min_image_count)
                    .min(surface_capabilities.max_image_count)
            )
            .image_format(surface_formats.first().context("No surface formats")?.format)
            .image_color_space(surface_formats.first().context("No surface formats")?.color_space)
            .image_extent(surface_capabilities.current_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .queue_family_indices(&queue_families)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO);
    
        unsafe { swapchain_loader.create_swapchain(&create_info, None) }?
    };

    let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain) }?;

    let mut swapchain_image_views = Vec::with_capacity(swapchain_images.len());

    for image in &swapchain_images {
        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);

        let create_info = vk::ImageViewCreateInfo::builder()
            .image(*image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(vk::Format::B8G8R8A8_UNORM)
            .subresource_range(*subresource_range);

        let image_view = unsafe { device.create_image_view(&create_info, None) }?;

        swapchain_image_views.push(image_view);
    }

    let render_pass = {
        let format = unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface) }?
            .first()
            .context("No swapchain formats")?
            .format;

        let attachments = [
            vk::AttachmentDescription::builder()
                .format(format)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .samples(vk::SampleCountFlags::TYPE_1)
                .build()
        ];

        let attachment_references = [
            vk::AttachmentReference::builder()
                .attachment(0)
                .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .build()
        ];

        let subpasses = [
            vk::SubpassDescription::builder()
                .color_attachments(&attachment_references)
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .build()
        ];

        let subpass_dependencies = [
            vk::SubpassDependency::builder()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_subpass(0)
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(
                    vk::AccessFlags::COLOR_ATTACHMENT_READ |
                    vk::AccessFlags::COLOR_ATTACHMENT_WRITE
                )
                .build()
        ];

        let create_info = vk::RenderPassCreateInfo::builder()
                .attachments(&attachments)
                .subpasses(&subpasses)
                .dependencies(&subpass_dependencies);

        unsafe { device.create_render_pass(&create_info, None) }?
    };

    {
        let vert_module = {
            let code = read_shader("assets/shaders/test/bin/vert.spv")?;

            let create_info = vk::ShaderModuleCreateInfo::builder()
                .code(&code);

            unsafe { device.create_shader_module(&create_info, None) }?
        };

        let frag_module = {
            let code = read_shader("assets/shaders/test/bin/frag.spv")?;

            let create_info = vk::ShaderModuleCreateInfo::builder()
                .code(&code);

            unsafe { device.create_shader_module(&create_info, None) }?
        };

        let entry_point = std::ffi::CString::new("main")?;

        let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(&entry_point);

        let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(&entry_point);

        let _shader_stages = vec![
            vert_stage.build(),
            frag_stage.build()
        ];

        unsafe {
            device.destroy_shader_module(frag_module, None);
            device.destroy_shader_module(vert_module, None);
        }
    }

    event_loop.run(move |event, _| {
        match event {
            _ => ()
        }
    })?;


    for image_view in &swapchain_image_views {
        unsafe { device.destroy_image_view(*image_view, None); }
    }

    unsafe {
        device.destroy_render_pass(render_pass, None);
        swapchain_loader.destroy_swapchain(swapchain, None);
        device.destroy_device(None);
        surface_loader.destroy_surface(surface, None);
        instance.destroy_instance(None);
    };

    Ok(())
}

