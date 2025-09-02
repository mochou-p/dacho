// dacho/crates/dacho_renderer/src/lib.rs

#![expect(
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
    reason = "most of vulkan is unsafe"
)]

use std::{array, ffi, fs};

use ash::khr::{surface, swapchain};
use ash::vk;

use ash_window::create_surface;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub use ash;


const MAX_FRAMES_IN_FLIGHT: usize = 4;

pub struct Vulkan {
    entry:           ash::Entry,
    instance:        ash::Instance,
    physical_device: vk::PhysicalDevice,
    device:          ash::Device,
    queue:           vk::Queue,
    ext_surface:     surface::Instance,
    ext_swapchain:   swapchain::Device
}

impl Vulkan {
    #[must_use]
    pub fn new(instance_extensions: &'static [*const ffi::c_char]) -> Self {
        let entry = unsafe { ash::Entry::load() }
            .unwrap();

        let application_info = vk::ApplicationInfo::default()
            .application_name(c"dacho")
            .api_version(vk::API_VERSION_1_3);
        let instance_create_info = vk::InstanceCreateInfo::default()
            .enabled_extension_names(instance_extensions)
            .application_info(&application_info);
        let instance = unsafe { entry.create_instance(&instance_create_info, None) }
            .unwrap();

        let physical_device = unsafe { instance.enumerate_physical_devices() }
            .unwrap()
            .swap_remove(0);

        let queue_create_infos = [
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(0)
                .queue_priorities(&[1.0])
        ];
        let enabled_extension_names = Box::leak(Box::new([vk::KHR_SWAPCHAIN_NAME.as_ptr()]));
        let enabled_features = vk::PhysicalDeviceFeatures::default()
            .logic_op(true);
        let mut vulkan13_extensions = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);
        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(enabled_extension_names)
            .enabled_features(&enabled_features)
            .push_next(&mut vulkan13_extensions);
        let device = unsafe { instance.create_device(physical_device, &device_create_info, None) }
            .unwrap();

        let queue = unsafe { device.get_device_queue(0, 0) };

        let ext_surface   = surface::Instance::new(&entry,    &instance);
        let ext_swapchain = swapchain::Device::new(&instance, &device  );

        Self {
            entry,
            instance,
            physical_device,
            device,
            queue,
            ext_surface,
            ext_swapchain
        }
    }

    #[must_use]
    pub fn new_renderer<H: HasDisplayHandle + HasWindowHandle>(&self, handle: H, width: u32, height: u32, clear_color: [f32; 4]) -> Renderer {
        Renderer::new(self, handle, width, height, clear_color)
    }

    pub fn destroy_renderer(&self, renderer: Renderer) {
        renderer.destroy(self);
    }

    #[inline]
    pub fn device_wait_idle(&self) {
        unsafe { self.device.device_wait_idle() }
            .unwrap();
    }

    #[inline]
    pub fn render<F: Fn()>(&self, renderer: &mut Renderer, winit_pre_present_notify: F) {
        let in_flight_fence           = renderer.in_flight_fences          [renderer.frame_index];
        let image_ready_semaphore     = renderer.image_ready_semaphores    [renderer.frame_index];
        let render_finished_semaphore = renderer.render_finished_semaphores[renderer.frame_index];
        let command_buffer            = renderer.command_buffers           [renderer.frame_index];

        self.wait_for_and_reset_fences(in_flight_fence);
        self.reset_command_buffer(command_buffer);

        let image_index = self.acquire_next_image(renderer.swapchain, image_ready_semaphore);
        let image       = renderer.swapchain_images[image_index as usize];

        self.with_command_buffer(command_buffer, || {
            self.with_image_memory_barriers(image, renderer, command_buffer, || {
                self.with_rendering(renderer, image_index, command_buffer, || {
                    unsafe {
                        self.device.cmd_set_viewport(command_buffer, 0, &renderer.viewports);
                        self.device.cmd_set_scissor(command_buffer, 0, &renderer.scissors);
                        self.device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, renderer.pipeline);
                        self.device.cmd_draw(command_buffer, 3, 1, 0, 0);
                    }
                });
            });
        });

        winit_pre_present_notify();

        self.submit_and_present(
            image_ready_semaphore,
            command_buffer,
            render_finished_semaphore,
            in_flight_fence,
            renderer.swapchain,
            image_index
        );

        renderer.frame_index = (renderer.frame_index + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    #[inline]
    fn wait_for_and_reset_fences(&self, in_flight_fence: vk::Fence) {
        let in_flight_fences = [in_flight_fence];

        unsafe { self.device.wait_for_fences(&in_flight_fences, true, u64::MAX) }
            .unwrap();
        unsafe { self.device.reset_fences(&in_flight_fences) }
            .unwrap();
    }

    #[inline]
    fn reset_command_buffer(&self, command_buffer: vk::CommandBuffer) {
        unsafe { self.device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::RELEASE_RESOURCES) }
            .unwrap();
    }

    #[inline]
    fn acquire_next_image(&self, swapchain: vk::SwapchainKHR, image_ready_semaphore: vk::Semaphore) -> u32 {
        let (image_index, _) = unsafe { self.ext_swapchain.acquire_next_image(swapchain, u64::MAX, image_ready_semaphore, vk::Fence::null()) }
            .unwrap();

        image_index
    }

    #[inline]
    fn with_command_buffer<F: Fn()>(&self, command_buffer: vk::CommandBuffer, f: F) {
        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe { self.device.begin_command_buffer(command_buffer, &begin_info) }
            .unwrap();

        f();

        unsafe { self.device.end_command_buffer(command_buffer) }
            .unwrap();
    }

    #[inline]
    fn with_image_memory_barriers<F: Fn()>(&self, image: vk::Image, renderer: &Renderer, command_buffer: vk::CommandBuffer, f: F) {
        let rendering_image_memory_barriers = [
            vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::NONE)
                .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(image)
                .subresource_range(renderer.subresource_range)
        ];
        let rendering_dependency_info = vk::DependencyInfo::default()
            .image_memory_barriers(&rendering_image_memory_barriers);
        unsafe { self.device.cmd_pipeline_barrier2(command_buffer, &rendering_dependency_info); }

        f();

        let presenting_image_memory_barriers = [
            vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::BOTTOM_OF_PIPE)
                .dst_access_mask(vk::AccessFlags2::NONE)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .image(image)
                .subresource_range(renderer.subresource_range)
        ];
        let presenting_dependency_info = vk::DependencyInfo::default()
            .image_memory_barriers(&presenting_image_memory_barriers);
        unsafe { self.device.cmd_pipeline_barrier2(command_buffer, &presenting_dependency_info); }
    }

    #[inline]
    fn with_rendering<F: Fn()>(&self, renderer: &Renderer, image_index: u32, command_buffer: vk::CommandBuffer, f: F) {
        let color_attachments = [
            vk::RenderingAttachmentInfo::default()
                .image_view(renderer.swapchain_image_views[image_index as usize])
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .clear_value(renderer.clear_value)
        ];
        let rendering_info = vk::RenderingInfo::default()
            .render_area(renderer.image_extent.into())
            .layer_count(1)
            .color_attachments(&color_attachments);

        unsafe { self.device.cmd_begin_rendering(command_buffer, &rendering_info); }

        f();

        unsafe { self.device.cmd_end_rendering(command_buffer); }
    }

    #[inline]
    fn submit_and_present(
        &self,
        image_ready_semaphore:     vk::Semaphore,
        command_buffer:            vk::CommandBuffer,
        render_finished_semaphore: vk::Semaphore,
        in_flight_fence:           vk::Fence,
        swapchain:                 vk::SwapchainKHR,
        image_index:               u32
    ) {
        let render_finished_semaphores = [render_finished_semaphore];
        let swapchains                 = [swapchain];
        let image_indices              = [image_index];

        let image_ready_semaphore_infos = [
            vk::SemaphoreSubmitInfo::default()
                .semaphore(image_ready_semaphore)
                .value(0)
                .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT_KHR)
        ];
        let command_buffer_infos = [
            vk::CommandBufferSubmitInfo::default()
                .command_buffer(command_buffer)
        ];
        let render_finished_semaphore_infos = [
            vk::SemaphoreSubmitInfo::default()
                .semaphore(render_finished_semaphore)
                .value(0)
                .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS_KHR)
        ];
        let submit_infos = [
            vk::SubmitInfo2::default()
                .wait_semaphore_infos(&image_ready_semaphore_infos)
                .command_buffer_infos(&command_buffer_infos)
                .signal_semaphore_infos(&render_finished_semaphore_infos)
        ];
        unsafe { self.device.queue_submit2(self.queue, &submit_infos, in_flight_fence) }
            .unwrap();

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(&render_finished_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);
        unsafe { self.ext_swapchain.queue_present(self.queue, &present_info) }
            .unwrap();
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            self.device  .destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

pub struct Renderer {
        surface:                    vk::SurfaceKHR,
        image_extent:               vk::Extent2D,
        swapchain:                  vk::SwapchainKHR,
        swapchain_images:           Vec<vk::Image>,
        subresource_range:          vk::ImageSubresourceRange,
        swapchain_image_views:      Vec<vk::ImageView>,
    pub clear_value:                vk::ClearValue,
        frame_index:                usize,
        image_ready_semaphores:     [vk::Semaphore;     MAX_FRAMES_IN_FLIGHT],
        render_finished_semaphores: [vk::Semaphore;     MAX_FRAMES_IN_FLIGHT],
        in_flight_fences:           [vk::Fence;         MAX_FRAMES_IN_FLIGHT],
        command_pool:               vk::CommandPool,
        command_buffers:            [vk::CommandBuffer; MAX_FRAMES_IN_FLIGHT],
        viewports:                  [vk::Viewport; 1],
        scissors:                   [vk::Rect2D;   1],
        pipeline_layout:            vk::PipelineLayout,
        pipeline:                   vk::Pipeline
}

impl Renderer {
    #[must_use]
    fn new<H: HasDisplayHandle + HasWindowHandle>(vk: &Vulkan, handle: H, width: u32, height: u32, clear_color: [f32; 4]) -> Self {
        let rdh = handle
            .display_handle()
            .unwrap()
            .into();
        let rwh = handle
            .window_handle()
            .unwrap()
            .into();
        let surface = unsafe { create_surface(&vk.entry, &vk.instance, rdh, rwh, None) }
            .unwrap();

        let surface_capabilities = unsafe { vk.ext_surface.get_physical_device_surface_capabilities(vk.physical_device, surface) }
            .unwrap();
        let image_extent = vk::Extent2D { width, height };
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .old_swapchain(vk::SwapchainKHR::null())
            .surface(surface)
            .image_format(vk::Format::R8G8B8A8_UNORM)
            .image_extent(image_extent)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_array_layers(1)
            .min_image_count(u32::try_from(MAX_FRAMES_IN_FLIGHT).unwrap())
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .pre_transform(surface_capabilities.current_transform)
            .clipped(true)
            .present_mode(vk::PresentModeKHR::FIFO);
        let swapchain = unsafe { vk.ext_swapchain.create_swapchain(&swapchain_create_info, None) }
            .unwrap();

        let swapchain_images = unsafe { vk.ext_swapchain.get_swapchain_images(swapchain) }
            .unwrap();

        let subresource_range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);
        let swapchain_image_views = swapchain_images
            .iter()
            .map(|image| {
                let image_view_create_info = vk::ImageViewCreateInfo::default()
                    .image(*image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(vk::Format::R8G8B8A8_UNORM)
                    .subresource_range(subresource_range);

                let image_view = unsafe { vk.device.create_image_view(&image_view_create_info, None) }
                    .unwrap();

                image_view
            })
            .collect();

        let clear_value = vk::ClearValue { color: vk::ClearColorValue {
            float32: [clear_color[0], clear_color[1], clear_color[2], clear_color[3]]
        } };

        let frame_index = 0;

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();

        let image_ready_semaphores = array::from_fn(|_| {
            unsafe { vk.device.create_semaphore(&semaphore_create_info, None) }
                .unwrap()
        });
        let render_finished_semaphores = array::from_fn(|_| {
            unsafe { vk.device.create_semaphore(&semaphore_create_info, None) }
                .unwrap()
        });

        let fence_create_info = vk::FenceCreateInfo::default()
            .flags(vk::FenceCreateFlags::SIGNALED);
        let in_flight_fences = array::from_fn(|_| {
            unsafe { vk.device.create_fence(&fence_create_info, None) }
                .unwrap()
        });

        let command_pool_create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::TRANSIENT | vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(0);
        let command_pool = unsafe { vk.device.create_command_pool(&command_pool_create_info, None) }
            .unwrap();

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(u32::try_from(MAX_FRAMES_IN_FLIGHT).unwrap());
        let command_buffers = unsafe { vk.device.allocate_command_buffers(&command_buffer_allocate_info) }
            .unwrap()
            .try_into()
            .unwrap();

        let vertex_code   = read_spirv("examples/usage/assets/shaders/test/vert.glsl");
        let fragment_code = read_spirv("examples/usage/assets/shaders/test/frag.glsl");
        let vertex_module_create_info = vk::ShaderModuleCreateInfo::default()
            .code(&vertex_code);
        let vertex_module = unsafe { vk.device.create_shader_module(&vertex_module_create_info, None) }
            .unwrap();
        let fragment_module_create_info = vk::ShaderModuleCreateInfo::default()
            .code(&fragment_code);
        let fragment_module = unsafe { vk.device.create_shader_module(&fragment_module_create_info, None) }
            .unwrap();
        let entry_point = c"main";
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vertex_module)
                .name(entry_point),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment_module)
                .name(entry_point)
        ];
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&[])
            .vertex_attribute_descriptions(&[]);
        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewports = [
            vk::Viewport {
                x:         0.0,          y:         0.0,
                width:     width as f32, height:    height as f32,
                min_depth: 0.0,          max_depth: 1.0
            }
        ];
        let scissors = [
            image_extent.into()
        ];
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);
        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .line_width(1.0);
        let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let color_blend_attachments = [
            vk::PipelineColorBlendAttachmentState::default()
                .blend_enable(false)
                .color_write_mask(vk::ColorComponentFlags::RGBA)
        ];
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(false)
            .depth_write_enable(false)
            .depth_compare_op(vk::CompareOp::LESS_OR_EQUAL)
            .depth_bounds_test_enable(false)
            .stencil_test_enable(false);
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
            .logic_op_enable(false)
            .attachments(&color_blend_attachments);
        let dynamic_state = vk::PipelineDynamicStateCreateInfo::default()
            .dynamic_states(&[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR]);
        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = unsafe { vk.device.create_pipeline_layout(&pipeline_layout_create_info, None) }
            .unwrap();
        let mut rendering_info = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&[vk::Format::R8G8B8A8_UNORM]);
        let pipeline_create_infos = [
                vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(&vertex_input_state)
                .input_assembly_state(&input_assembly_state)
                .viewport_state(&viewport_state)
                .rasterization_state(&rasterization_state)
                .multisample_state(&multisample_state)
                .depth_stencil_state(&depth_stencil_state)
                .color_blend_state(&color_blend_state)
                .dynamic_state(&dynamic_state)
                .layout(pipeline_layout)
                .push_next(&mut rendering_info)
        ];
        let pipeline = unsafe { vk.device.create_graphics_pipelines(vk::PipelineCache::null(), &pipeline_create_infos, None) }
            .unwrap()
            .swap_remove(0);

        unsafe { vk.device.destroy_shader_module(  vertex_module, None); }
        unsafe { vk.device.destroy_shader_module(fragment_module, None); }

        Self {
            surface,
            image_extent,
            swapchain,
            swapchain_images,
            subresource_range,
            swapchain_image_views,
            clear_value,
            frame_index,
            image_ready_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            command_pool,
            command_buffers,
            viewports,
            scissors,
            pipeline_layout,
            pipeline
        }
    }

    fn destroy(self, vk: &Vulkan) {
        unsafe {
            vk.device.destroy_pipeline(self.pipeline, None);
            vk.device.destroy_pipeline_layout(self.pipeline_layout, None);
            vk.device.destroy_command_pool(self.command_pool, None);

            self.in_flight_fences
                .into_iter()
                .for_each(|fence| vk.device.destroy_fence(fence, None));

            self.render_finished_semaphores
                .into_iter()
                .for_each(|semaphore| vk.device.destroy_semaphore(semaphore, None));
            self.image_ready_semaphores
                .into_iter()
                .for_each(|semaphore| vk.device.destroy_semaphore(semaphore, None));

            self.swapchain_image_views
                .into_iter()
                .for_each(|image_view| vk.device.destroy_image_view(image_view, None));

            vk.ext_swapchain.destroy_swapchain(self.swapchain, None);
            vk.ext_surface  .destroy_surface(self.surface, None);
        }
    }
}

fn read_spirv(filepath: &str) -> Vec<u32> {
    let bytes = fs::read(format!("{filepath}.spv")).unwrap();

    #[cfg(debug_assertions)]
    assert!((bytes.len() % 4) == 0, "invalid SPIR-V file");

    let mut words = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks(4) {
        let mut word = [0_u8; 4];
        word.copy_from_slice(chunk);
        words.push(u32::from_ne_bytes(word));
    }

    words
}

