// dacho/crates/dacho_renderer/src/lib.rs

#![expect(
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
    reason = "most of vulkan is unsafe"
)]

use std::array;
use std::ffi::c_char;

use ash::{Device, Entry, Instance};
use ash::khr::surface::Instance as SurfaceInstance;
use ash::khr::swapchain::Device as SwapchainDevice;
use ash::vk::{AccessFlags2, ApplicationInfo, AttachmentLoadOp, AttachmentStoreOp, ClearColorValue, ClearValue, CompositeAlphaFlagsKHR, CommandBuffer, CommandBufferAllocateInfo, CommandBufferUsageFlags, CommandBufferBeginInfo, CommandBufferLevel, CommandBufferResetFlags, CommandBufferSubmitInfo, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo, DependencyInfo, DeviceCreateInfo, DeviceQueueCreateInfo, Extent2D, Fence, FenceCreateFlags, FenceCreateInfo, Format, Image, ImageAspectFlags, ImageLayout, ImageMemoryBarrier2, ImageSubresourceRange, ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, InstanceCreateInfo, PhysicalDevice, PhysicalDeviceVulkan13Features, PipelineStageFlags2, PresentInfoKHR, PresentModeKHR, RenderingAttachmentInfo, RenderingInfo, Semaphore, SemaphoreCreateInfo, SemaphoreSubmitInfo, SubmitInfo2, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR, Queue, API_VERSION_1_3, KHR_SWAPCHAIN_NAME};

use ash_window::create_surface;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};


const MAX_FRAMES_IN_FLIGHT: usize = 4;

pub struct Vulkan {
    entry:           Entry,
    instance:        Instance,
    physical_device: PhysicalDevice,
    device:          Device,
    queue:           Queue,
    ext_surface:     SurfaceInstance,
    ext_swapchain:   SwapchainDevice
}

impl Vulkan {
    #[must_use]
    pub fn new(instance_extensions: &'static [*const c_char]) -> Self {
        let entry = unsafe { Entry::load() }
            .unwrap();

        let application_info = ApplicationInfo::default()
            .api_version(API_VERSION_1_3);
        let instance_create_info = InstanceCreateInfo::default()
            .enabled_extension_names(instance_extensions)
            .application_info(&application_info);
        let instance = unsafe { entry.create_instance(&instance_create_info, None) }
            .unwrap();

        let physical_device = unsafe { instance.enumerate_physical_devices() }
            .unwrap()
            .swap_remove(0);

        let queue_create_infos = [
            DeviceQueueCreateInfo::default()
                .queue_family_index(0)
                .queue_priorities(&[1.0])
        ];
        let device_extensions = Box::leak(Box::new([KHR_SWAPCHAIN_NAME.as_ptr()]));
        let mut vulkan13_extensions = PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);
        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(device_extensions)
            .push_next(&mut vulkan13_extensions);
        let device = unsafe { instance.create_device(physical_device, &device_create_info, None) }
            .unwrap();

        let queue = unsafe { device.get_device_queue(0, 0) };

        let ext_surface   = SurfaceInstance::new(&entry,    &instance);
        let ext_swapchain = SwapchainDevice::new(&instance, &device  );

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

    #[inline]
    #[must_use]
    pub fn new_renderer<H: HasDisplayHandle + HasWindowHandle>(&self, handle: H, width: u32, height: u32, clear_color: [f32; 4]) -> Renderer {
        Renderer::new(self, handle, width, height, clear_color)
    }

    #[inline]
    pub fn destroy_renderer(&self, renderer: Renderer) {
        renderer.destroy(self);
    }

    #[inline]
    pub fn device_wait_idle(&self) {
        unsafe { self.device.device_wait_idle() }
            .unwrap();
    }

    pub fn render<F: Fn()>(&self, renderer: &mut Renderer, pre_present_notify: F) {
        let in_flight_fence           = renderer.in_flight_fences          [renderer.frame_index];
        let image_ready_semaphore     = renderer.image_ready_semaphores    [renderer.frame_index];
        let render_finished_semaphore = renderer.render_finished_semaphores[renderer.frame_index];
        let command_buffer            = renderer.command_buffers           [renderer.frame_index];

        let fences = [in_flight_fence];
        unsafe { self.device.wait_for_fences(&fences, true, u64::MAX) }
            .unwrap();
        unsafe { self.device.reset_fences(&fences) }
            .unwrap();

        unsafe { self.device.reset_command_buffer(command_buffer, CommandBufferResetFlags::RELEASE_RESOURCES) }
            .unwrap();

        let (image_index, _) = unsafe { self.ext_swapchain.acquire_next_image(renderer.swapchain, u64::MAX, image_ready_semaphore, Fence::null()) }
            .unwrap();

        let render_finished_semaphores = [render_finished_semaphore];
        let swapchains                 = [renderer.swapchain];
        let image_indices              = [image_index];

        let begin_info = CommandBufferBeginInfo::default()
            .flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe { self.device.begin_command_buffer(command_buffer, &begin_info) }
            .unwrap();

        let image = renderer.swapchain_images[image_index as usize];

        let rendering_image_memory_barriers = [
            ImageMemoryBarrier2::default()
                .src_stage_mask(PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(AccessFlags2::NONE)
                .dst_stage_mask(PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(ImageLayout::UNDEFINED)
                .new_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(image)
                .subresource_range(renderer.subresource_range)
        ];
        let rendering_dependency_info = DependencyInfo::default()
            .image_memory_barriers(&rendering_image_memory_barriers);
        unsafe { self.device.cmd_pipeline_barrier2(command_buffer, &rendering_dependency_info); }

        let color_attachments = [
            RenderingAttachmentInfo::default()
                .image_view(renderer.swapchain_image_views[image_index as usize])
                .image_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(AttachmentLoadOp::CLEAR)
                .store_op(AttachmentStoreOp::STORE)
                .clear_value(renderer.clear_value)
        ];
        let rendering_info = RenderingInfo::default()
            .render_area(renderer.image_extent.into())
            .layer_count(1)
            .color_attachments(&color_attachments);
        unsafe { self.device.cmd_begin_rendering(command_buffer, &rendering_info); }

        // TODO: triangles here!

        unsafe { self.device.cmd_end_rendering(command_buffer); }

        let presenting_image_memory_barriers = [
            ImageMemoryBarrier2::default()
                .src_stage_mask(PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .dst_stage_mask(PipelineStageFlags2::BOTTOM_OF_PIPE)
                .dst_access_mask(AccessFlags2::NONE)
                .old_layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(ImageLayout::PRESENT_SRC_KHR)
                .image(image)
                .subresource_range(renderer.subresource_range)
        ];
        let presenting_dependency_info = DependencyInfo::default()
            .image_memory_barriers(&presenting_image_memory_barriers);
        unsafe { self.device.cmd_pipeline_barrier2(command_buffer, &presenting_dependency_info); }


        unsafe { self.device.end_command_buffer(command_buffer) }
            .unwrap();

        pre_present_notify();

        let image_ready_semaphore_infos = [
            SemaphoreSubmitInfo::default()
                .semaphore(image_ready_semaphore)
                .value(0)
                .stage_mask(PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT_KHR)
        ];
        let command_buffer_infos = [
            CommandBufferSubmitInfo::default()
                .command_buffer(command_buffer)
        ];
        let render_finished_semaphore_infos = [
            SemaphoreSubmitInfo::default()
                .semaphore(render_finished_semaphore)
                .value(0)
                .stage_mask(PipelineStageFlags2::ALL_COMMANDS_KHR)
        ];
        let submit_infos = [
            SubmitInfo2::default()
                .wait_semaphore_infos(&image_ready_semaphore_infos)
                .command_buffer_infos(&command_buffer_infos)
                .signal_semaphore_infos(&render_finished_semaphore_infos)
        ];
        unsafe { self.device.queue_submit2(self.queue, &submit_infos, in_flight_fence) }
            .unwrap();

        let present_info = PresentInfoKHR::default()
            .wait_semaphores(&render_finished_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);
        unsafe { self.ext_swapchain.queue_present(self.queue, &present_info) }
            .unwrap();

        renderer.frame_index = (renderer.frame_index + 1) % MAX_FRAMES_IN_FLIGHT;
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
        surface:                    SurfaceKHR,
        image_extent:               Extent2D,
        swapchain:                  SwapchainKHR,
        swapchain_images:           Vec<Image>,
        subresource_range:          ImageSubresourceRange,
        swapchain_image_views:      Vec<ImageView>,
    pub clear_value:                ClearValue,
        frame_index:                usize,
        image_ready_semaphores:     [Semaphore;     MAX_FRAMES_IN_FLIGHT],
        render_finished_semaphores: [Semaphore;     MAX_FRAMES_IN_FLIGHT],
        in_flight_fences:           [Fence;         MAX_FRAMES_IN_FLIGHT],
        command_pool:               CommandPool,
        command_buffers:            [CommandBuffer; MAX_FRAMES_IN_FLIGHT]
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
        let image_extent = Extent2D { width, height };
        let swapchain_create_info = SwapchainCreateInfoKHR::default()
            .old_swapchain(SwapchainKHR::null())
            .surface(surface)
            .image_format(Format::R8G8B8A8_UNORM)
            .image_extent(image_extent)
            .image_usage(ImageUsageFlags::COLOR_ATTACHMENT)
            .image_array_layers(1)
            .min_image_count(u32::try_from(MAX_FRAMES_IN_FLIGHT).unwrap())
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .pre_transform(surface_capabilities.current_transform)
            .clipped(true)
            .present_mode(PresentModeKHR::FIFO);
        let swapchain = unsafe { vk.ext_swapchain.create_swapchain(&swapchain_create_info, None) }
            .unwrap();

        let swapchain_images = unsafe { vk.ext_swapchain.get_swapchain_images(swapchain) }
            .unwrap();

        let subresource_range = ImageSubresourceRange::default()
            .aspect_mask(ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);
        let swapchain_image_views = swapchain_images
            .iter()
            .map(|image| {
                let image_view_create_info = ImageViewCreateInfo::default()
                    .image(*image)
                    .view_type(ImageViewType::TYPE_2D)
                    .format(Format::R8G8B8A8_UNORM)
                    .subresource_range(subresource_range);

                let image_view = unsafe { vk.device.create_image_view(&image_view_create_info, None) }
                    .unwrap();

                image_view
            })
            .collect::<Vec<ImageView>>();

        let clear_value = ClearValue { color: ClearColorValue {
            float32: [clear_color[0], clear_color[1], clear_color[2], clear_color[3]]
        } };

        let frame_index = 0;

        let semaphore_create_info = SemaphoreCreateInfo::default();

        let image_ready_semaphores = array::from_fn(|_| {
            unsafe { vk.device.create_semaphore(&semaphore_create_info, None) }
                .unwrap()
        });
        let render_finished_semaphores = array::from_fn(|_| {
            unsafe { vk.device.create_semaphore(&semaphore_create_info, None) }
                .unwrap()
        });

        let fence_create_info = FenceCreateInfo::default()
            .flags(FenceCreateFlags::SIGNALED);
        let in_flight_fences = array::from_fn(|_| {
            unsafe { vk.device.create_fence(&fence_create_info, None) }
                .unwrap()
        });

        let command_pool_create_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::TRANSIENT | CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(0);
        let command_pool = unsafe { vk.device.create_command_pool(&command_pool_create_info, None) }
            .unwrap();

        let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(u32::try_from(MAX_FRAMES_IN_FLIGHT).unwrap());
        let command_buffers = unsafe { vk.device.allocate_command_buffers(&command_buffer_allocate_info) }
            .unwrap()
            .try_into()
            .unwrap();

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
            command_buffers
        }
    }

    fn destroy(self, vk: &Vulkan) {
        unsafe {
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

