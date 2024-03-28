// dacho/src/main.rs

use anyhow::{Context, Result};

use ash::{
    extensions::khr::{Surface, Swapchain},
    vk
};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder}
};

const VALIDATION_LAYERS: [&'static str; 1] = [
    "VK_LAYER_KHRONOS_validation"
];

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity:        vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type:            vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data:  *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data:     *mut   std::ffi::c_void,
) -> vk::Bool32 {
    static mut NUMBER: usize = 0;
    NUMBER += 1;

    let severity = match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "\x1b[36;1m[Verbose]",
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO    => "\x1b[36;1m[Info]",
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "\x1b[33;1m[Warning]",
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR   => "\x1b[31;1m[Error]",
        _                                              => "\x1b[35;1m[Unknown]"
    };

    let kind = match message_type {
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL     => "\x1b[36;1m[General]",
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "\x1b[33;1m[Performance]",
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION  => "\x1b[31;1m[Validation]",
            _                                              => "\x1b[35;1m[Unknown]"
    };

    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);

    let mut msg = format!(
        "\n\x1b[33m({NUMBER}) \x1b[1m{kind} \x1b[1m{severity}\x1b[0m\n{:?}\n",
        message
    );

    if let Some(index) = msg.find("The Vulkan spec states:") {
        msg.insert_str(index, "\n\x1b[35;1m->\x1b[0m ");
    }

    println!("{msg}");

    vk::FALSE
}

struct Renderer {
    _entry:                  ash::Entry,
    instance:                ash::Instance,
    debug_utils_loader:      ash::extensions::ext::DebugUtils,
    debug_messenger:         ash::vk::DebugUtilsMessengerEXT,
    queue:                   vk::Queue,
    window:                  Window,
    surface_loader:          Surface,
    surface:                 vk::SurfaceKHR,
    device:                  ash::Device,
    swapchain_loader:        Swapchain,
    swapchain:               vk::SwapchainKHR,
    swapchain_image_count:   usize,
    swapchain_current_image: usize,
    swapchain_image_views:   Vec<vk::ImageView>,
    framebuffers:            Vec<vk::Framebuffer>,
    images_available:        Vec<vk::Semaphore>,
    images_finished:         Vec<vk::Semaphore>,
    may_begin_drawing:       Vec<vk::Fence>,
    render_pass:             vk::RenderPass,
    pipeline_layout:         vk::PipelineLayout,
    pipeline:                vk::Pipeline,
    command_pool:            vk::CommandPool,
    command_buffers:         Vec<vk::CommandBuffer>
}

fn debug_messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
    vk::DebugUtilsMessengerCreateInfoEXT {
        s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: std::ptr::null(),
        flags:  vk::DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity:
            // vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            // vk::DebugUtilsMessageSeverityFlagsEXT::INFO    |
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING |
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type:
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL     |
            vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE |
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(vulkan_debug_utils_callback),
        p_user_data:       std::ptr::null_mut()
    }
}

fn debug_utils(
    entry:    &ash::Entry,
    instance: &ash::Instance,
) -> Result<(ash::extensions::ext::DebugUtils, vk::DebugUtilsMessengerEXT)> {
    let debug_utils_loader = ash::extensions::ext::DebugUtils::new(entry, instance);

    let messenger = debug_messenger_create_info();

    let utils_messenger = unsafe {
        debug_utils_loader
            .create_debug_utils_messenger(&messenger, None)
    }?;

    Ok((debug_utils_loader, utils_messenger))
}

impl Renderer {
    fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let entry = ash::Entry::linked();

        let instance = {
            let application_info = vk::ApplicationInfo::builder()
                .api_version(vk::API_VERSION_1_3);

            let required_extensions = ash_window::enumerate_required_extensions(
                event_loop.raw_display_handle()
            )?;

            let mut extension_names = required_extensions.to_vec();

            let debug     = std::ffi::CString::new("VK_EXT_debug_utils")?;
            let debug_ptr = debug.as_ptr();

            extension_names.push(debug_ptr);

            let debug_utils_create_info = debug_messenger_create_info();

            let layers_raw: Vec<std::ffi::CString> = VALIDATION_LAYERS
                .iter()
                .map(|layer| std::ffi::CString::new(*layer).unwrap())
                .collect();

            let layer_names: Vec<*const i8> = layers_raw
                .iter()
                .map(|layer| layer.as_ptr())
                .collect();

            let create_info = vk::InstanceCreateInfo {
                s_type:  vk::StructureType::INSTANCE_CREATE_INFO,
                p_next: &debug_utils_create_info
                    as *const vk::DebugUtilsMessengerCreateInfoEXT
                    as *const std::ffi::c_void,
                flags:                       vk::InstanceCreateFlags::empty(),
                p_application_info:         &application_info.build(),
                pp_enabled_layer_names:      layer_names.as_ptr(),
                enabled_layer_count:         layer_names.len() as u32,
                pp_enabled_extension_names:  extension_names.as_ptr(),
                enabled_extension_count:     extension_names.len() as u32
            };

            unsafe { entry.create_instance(&create_info, None) }?
        };

        let (debug_utils_loader, debug_messenger) = debug_utils(&entry, &instance)?;

        let physical_device = unsafe { instance.enumerate_physical_devices() }?
            .into_iter()
            .next()
            .context("No physical devices")?;

        let device = {
            let queue_priorities = [
                1.0
            ];

            let extension_names = [
                Swapchain::name()
                    .as_ptr()
            ];

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

        let queue = unsafe { device.get_device_queue(0, 0) };

        let scale  = 70;
        let width  = 16 * scale;
        let height =  9 * scale;

        let window = WindowBuilder::new()
            .with_title("dacho")
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .build(event_loop)?;

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

            let queue_families = [
                0
            ];

            let swapchain_extent = vk::Extent2D::builder()
                .width(width)
                .height(height)
                .build();

            let create_info = vk::SwapchainCreateInfoKHR::builder()
                .surface(surface)
                .min_image_count(surface_capabilities.min_image_count + 1)
                .image_format(vk::Format::R5G6B5_UNORM_PACK16)
                .image_color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
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

        let swapchain_images        = unsafe { swapchain_loader.get_swapchain_images(swapchain) }?;
        let swapchain_image_count   = swapchain_images.len();
        let swapchain_current_image = 0;

        let mut swapchain_image_views = Vec::with_capacity(swapchain_image_count);

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
                .format(vk::Format::R5G6B5_UNORM_PACK16)
                .subresource_range(*subresource_range);

            let image_view = unsafe { device.create_image_view(&create_info, None) }?;

            swapchain_image_views.push(image_view);
        }

        let render_pass = {
            let format = unsafe {
                surface_loader.get_physical_device_surface_formats(
                    physical_device, surface
                )
            }?
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

        let mut framebuffers = Vec::with_capacity(swapchain_image_count);

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

            for _ in 0..swapchain_image_count {
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

            for _ in 0..swapchain_image_count {
                let fence = unsafe { device.create_fence(&create_info, None) }?;

                may_begin_drawing.push(fence);
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
                .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

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

            let pipeline = unsafe {
                device.create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &pipeline_infos,
                    None
                )
            }
                .expect("Error creating pipelines")[0];

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
                .command_buffer_count(swapchain_image_count as u32);

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
                        float32: [0.0, 0.0, 0.0, 1.0]
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

                device.cmd_draw(command_buffer, 3, 1, 0, 0);
                device.cmd_end_render_pass(command_buffer);
                device.end_command_buffer(command_buffer)?;
            }
        }

        Ok(
            Renderer {
                _entry: entry,
                instance,
                debug_utils_loader,
                debug_messenger,
                queue,
                window,
                surface_loader,
                surface,
                device,
                swapchain_loader,
                swapchain,
                swapchain_image_count,
                swapchain_current_image,
                swapchain_image_views,
                framebuffers,
                images_available,
                images_finished,
                may_begin_drawing,
                render_pass,
                pipeline_layout,
                pipeline,
                command_pool,
                command_buffers
            }
        )
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { self.device.device_wait_idle() }
            .expect("Device idle wait failed");

        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_pipeline(self.pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);
        }

        for fence in &self.may_begin_drawing {
            unsafe { self.device.destroy_fence(*fence, None); }
        }

        for semaphore in &self.images_available {
            unsafe { self.device.destroy_semaphore(*semaphore, None); }
        }

        for semaphore in &self.images_finished {
            unsafe { self.device.destroy_semaphore(*semaphore, None); }
        }

        for framebuffer in &self.framebuffers {
            unsafe { self.device.destroy_framebuffer(*framebuffer, None); }
        }

        for image_view in &self.swapchain_image_views {
            unsafe { self.device.destroy_image_view(*image_view, None); }
        }

        unsafe {
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
        }

        unsafe {
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_messenger, None);
        }

        unsafe { self.instance.destroy_instance(None); }
    }
}

fn read_spirv(path: &str) -> Result<Vec<u32>> {
    let bytes = &std::fs::read(path)?;
    let words = bytemuck::cast_slice::<u8, u32>(bytes);

    Ok(words.to_vec())
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut renderer = Renderer::new(&event_loop)?;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            },
            Event::AboutToWait => {
                unsafe { renderer.device.device_wait_idle() }
                    .expect("Device idle wait failed");

                renderer.window.request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                let (image_index, _) = unsafe {
                    renderer.swapchain_loader
                        .acquire_next_image(
                            renderer.swapchain,
                            std::u64::MAX,
                            renderer.images_available[renderer.swapchain_current_image],
                            vk::Fence::null()
                        )
                }
                    .expect("Acquiring next image failed");

                unsafe {
                    renderer.device.wait_for_fences(
                        &[renderer.may_begin_drawing[renderer.swapchain_current_image]],
                        true,
                        std::u64::MAX
                    )
                }
                    .expect("Waiting for fences failed");

                unsafe {
                    renderer.device.reset_fences(
                        &[renderer.may_begin_drawing[renderer.swapchain_current_image]]
                    )
                }
                    .expect("Resetting fences failed");

                let semaphores_available = [
                    renderer.images_available[renderer.swapchain_current_image]
                ];

                let waiting_stages = [
                    vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                ];

                let command_buffers = [
                    renderer.command_buffers[image_index as usize]
                ];

                let semaphores_finished = [
                    renderer.images_finished[renderer.swapchain_current_image]
                ];

                let submit_info = [
                    vk::SubmitInfo::builder()
                        .wait_semaphores(&semaphores_available)
                        .wait_dst_stage_mask(&waiting_stages)
                        .command_buffers(&command_buffers)
                        .signal_semaphores(&semaphores_finished)
                        .build()
                ];

                unsafe {
                    renderer.device.queue_submit(
                        renderer.queue,
                        &submit_info,
                        renderer.may_begin_drawing[renderer.swapchain_current_image]
                    )
                }
                    .expect("Submitting queue failed");

                let swapchains    = [renderer.swapchain];
                let image_indices = [image_index];

                let present_info = vk::PresentInfoKHR::builder()
                    .wait_semaphores(&semaphores_finished)
                    .swapchains(&swapchains)
                    .image_indices(&image_indices);

                unsafe {
                    renderer.swapchain_loader.queue_present(
                        renderer.queue,
                        &present_info
                    )
                }
                    .expect("Presenting queue failed");

                renderer.swapchain_current_image =
                    (renderer.swapchain_current_image + 1)
                    % renderer.swapchain_image_count;
            },
            _ => ()
        }
    })?;

    Ok(())
}

