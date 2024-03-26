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

fn read_spirv(path: &str) -> Result<Vec<u32>> {
    let bytes = &std::fs::read(path)?;
    let words = bytemuck::cast_slice::<u8, u32>(bytes);

    Ok(words.to_vec())
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

    let surface = unsafe {
        ash_window::create_surface(
            &entry,
            &instance,
            window.raw_display_handle(),
            window.raw_window_handle(),
            None
        )
    }?;

    let swapchain_loader = Swapchain::new(&instance, &device);

    let (swapchain, swapchain_extent) = {
        let surface_capabilities = unsafe {
            surface_loader.get_physical_device_surface_capabilities(
                physical_device, surface
            )
        }?;

        let surface_formats = unsafe {
            surface_loader.get_physical_device_surface_formats(
                physical_device, surface
            )
        }?;

        let queue_families = [0];

        let swapchain_extent = surface_capabilities.current_extent;

        let create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(
                3.max(surface_capabilities.min_image_count)
                    .min(surface_capabilities.max_image_count)
            )
            .image_format(surface_formats.first().context("No surface formats")?.format)
            .image_color_space(surface_formats.first().context("No surface formats")?.color_space)
            .image_extent(swapchain_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .queue_family_indices(&queue_families)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO);
    
        (
            unsafe { swapchain_loader.create_swapchain(&create_info, None) }?,
            swapchain_extent
        )
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

    let mut framebuffers = Vec::with_capacity(swapchain_images.len());

    for image_view in &swapchain_image_views {
        let attachments = [*image_view];

        let create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(swapchain_extent.width)
            .height(swapchain_extent.height)
            .layers(1);

        let framebuffer = unsafe { device.create_framebuffer(&create_info, None) }?;

        framebuffers.push(framebuffer);
    }

    let mut images_available = vec![];
    let mut images_finished  = vec![];

    {
        let create_info = vk::SemaphoreCreateInfo::builder();

        for _ in 0..swapchain_images.len() {
            let semaphore_available = unsafe { device.create_semaphore(&create_info, None) }?;
            let semaphore_finished  = unsafe { device.create_semaphore(&create_info, None) }?;

            images_available.push(semaphore_available);
            images_finished.push(semaphore_finished);
        }
    }

    let pipeline_layout = {
        let create_info = vk::PipelineLayoutCreateInfo::builder();

        unsafe { device.create_pipeline_layout(&create_info, None) }?
    };

    let pipeline = {
        let vert_module = {
            let code = read_spirv("assets/shaders/test/bin/vert.spv")?;

            let create_info = vk::ShaderModuleCreateInfo::builder()
                .code(&code);

            unsafe { device.create_shader_module(&create_info, None) }?
        };

        let frag_module = {
            let code = read_spirv("assets/shaders/test/bin/frag.spv")?;

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

        let stages = vec![
            vert_stage.build(),
            frag_stage.build()
        ]; 

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder();

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::POINT_LIST);

        let viewports = [
            vk::Viewport::builder()
                .x(0.0)
                .y(0.0)
                .width(swapchain_extent.width as f32)
                .height(swapchain_extent.height as f32)
                .min_depth(0.0)
                .max_depth(1.0)
                .build()
        ];

        let scissors = [
            vk::Rect2D::builder()
                .offset(
                    vk::Offset2D::builder()
                        .x(0)
                        .y(0)
                        .build()
                )
                .extent(swapchain_extent)
                .build()
        ];

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);

        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
            .line_width(1.0)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .cull_mode(vk::CullModeFlags::NONE)
            .polygon_mode(vk::PolygonMode::FILL);

        let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachments = [
            vk::PipelineColorBlendAttachmentState::builder()
                .blend_enable(true)
                .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
                .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .color_blend_op(vk::BlendOp::ADD)
                .src_alpha_blend_factor(vk::BlendFactor::SRC_ALPHA)
                .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .alpha_blend_op(vk::BlendOp::ADD)
                .color_write_mask(
                    vk::ColorComponentFlags::R |
                    vk::ColorComponentFlags::G |
                    vk::ColorComponentFlags::B |
                    vk::ColorComponentFlags::A
                )
                .build()
        ];

        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .attachments(&color_blend_attachments); 

        let pipeline_infos = [
            vk::GraphicsPipelineCreateInfo::builder()
                .stages(&stages)
                .vertex_input_state(&vertex_input_state)
                .input_assembly_state(&input_assembly_state)
                .viewport_state(&viewport_state)
                .rasterization_state(&rasterization_state)
                .multisample_state(&multisample_state)
                .color_blend_state(&color_blend_state)
                .layout(pipeline_layout)
                .render_pass(render_pass)
                .subpass(0)
                .build()
        ];

        let pipeline = unsafe { device.create_graphics_pipelines(
            vk::PipelineCache::null(),
            &pipeline_infos,
            None
        ) }.expect("Error creating pipelines")[0];

        unsafe {
            device.destroy_shader_module(frag_module, None);
            device.destroy_shader_module(vert_module, None);
        }

        pipeline
    };

    let command_pool = {
        let create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(0)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        unsafe { device.create_command_pool(&create_info, None) }?
    };

    let command_buffers = {
        let allocate_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(framebuffers.len() as u32);

        unsafe { device.allocate_command_buffers(&allocate_info) }?
    };

    for (i, &command_buffer) in command_buffers.iter().enumerate() {
        {
            let begin_info = vk::CommandBufferBeginInfo::builder();

            unsafe { device.begin_command_buffer(command_buffer, &begin_info) }?;
        }

        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 1.0, 1.0]
                }
            }
        ];

        let begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(render_pass)
            .framebuffer(framebuffers[i])
            .render_area(
                vk::Rect2D::builder()
                    .offset(
                        vk::Offset2D::builder()
                            .x(0)
                            .y(0)
                            .build()
                    )
                    .extent(swapchain_extent)
                    .build()
            )
            .clear_values(&clear_values);

        unsafe {
            device.cmd_begin_render_pass(
                command_buffer,
                &begin_info,
                vk::SubpassContents::INLINE
            );

            device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline
            );

            device.cmd_draw(command_buffer, 1, 1, 0, 0);
            device.cmd_end_render_pass(command_buffer);
            device.end_command_buffer(command_buffer)?;
        }
    }



    event_loop.run(move |event, _| {
        match event {
            _ => ()
        }
    })?;

    unsafe {
        device.destroy_command_pool(command_pool, None);
        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_render_pass(render_pass, None);
    }

    for semaphore in &images_available {
        unsafe { device.destroy_semaphore(*semaphore, None); }
    }

    for semaphore in &images_finished {
        unsafe { device.destroy_semaphore(*semaphore, None); }
    }

    for framebuffer in &framebuffers {
        unsafe { device.destroy_framebuffer(*framebuffer, None); }
    }

    for image_view in &swapchain_image_views {
        unsafe { device.destroy_image_view(*image_view, None); }
    }

    unsafe {
        swapchain_loader.destroy_swapchain(swapchain, None);
        device.destroy_device(None);
        surface_loader.destroy_surface(surface, None);
        instance.destroy_instance(None);
    }

    Ok(())
}

