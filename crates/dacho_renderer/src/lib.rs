// dacho/crates/dacho_renderer/src/lib.rs

#![expect(
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
    reason = "most of vulkan is unsafe"
)]

use core::ffi::c_char;

use ash::{Device, Entry, Instance};
use ash::khr::surface::Instance as SurfaceInstance;
use ash::khr::swapchain::Device as SwapchainDevice;
use ash::vk::{ApplicationInfo, CompositeAlphaFlagsKHR, CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo, DeviceCreateInfo, DeviceQueueCreateInfo, Extent2D, Fence, FenceCreateFlags, FenceCreateInfo, Format, ImageUsageFlags, InstanceCreateInfo, PhysicalDevice, PresentModeKHR, Semaphore, SemaphoreCreateInfo, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR, Queue, API_VERSION_1_3, KHR_SWAPCHAIN_NAME};

use ash_window::create_surface;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};


pub struct Vulkan {
    entry:           Entry,
    instance:        Instance,
    physical_device: PhysicalDevice,
    device:          Device,
    _queue:          Queue,
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
        let device_extensions  = Box::leak(Box::new([KHR_SWAPCHAIN_NAME.as_ptr()]));
        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(device_extensions);
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
            _queue: queue,
            ext_surface,
            ext_swapchain
        }
    }

    #[must_use]
    pub fn new_renderer<H: HasDisplayHandle + HasWindowHandle>(&self, handle: H, width: u32, height: u32) -> Renderer {
        Renderer::new(self, handle, width, height)
    }

    pub fn destroy_renderer(&self, renderer: Renderer) {
        renderer.destroy(self);
    }

    pub fn device_wait_idle(&self) {
        unsafe { self.device.device_wait_idle().unwrap(); }
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
    surface:                   SurfaceKHR,
    swapchain:                 SwapchainKHR,
    image_ready_semaphore:     Semaphore,
    render_finished_semaphore: Semaphore,
    fence:                     Fence,
    command_pool:              CommandPool,
    _command_buffer:           CommandBuffer
}

impl Renderer {
    #[must_use]
    fn new<H: HasDisplayHandle + HasWindowHandle>(vk: &Vulkan, handle: H, width: u32, height: u32) -> Self {
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
        let swapchain_create_info = SwapchainCreateInfoKHR::default()
            .old_swapchain(SwapchainKHR::null())
            .surface(surface)
            .image_format(Format::R8G8B8A8_SRGB)
            .image_extent(Extent2D { width, height })
            .image_usage(ImageUsageFlags::TRANSFER_DST)
            .image_array_layers(1)
            .min_image_count(surface_capabilities.min_image_count)
            .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
            .pre_transform(surface_capabilities.current_transform)
            .clipped(true)
            .present_mode(PresentModeKHR::IMMEDIATE);
        let swapchain = unsafe { vk.ext_swapchain.create_swapchain(&swapchain_create_info, None) }
            .unwrap();

        let semaphore_create_info = SemaphoreCreateInfo::default();
        let image_ready_semaphore = unsafe { vk.device.create_semaphore(&semaphore_create_info, None) }
            .unwrap();
        let render_finished_semaphore = unsafe { vk.device.create_semaphore(&semaphore_create_info, None) }
            .unwrap();

        let fence_create_info = FenceCreateInfo::default()
            .flags(FenceCreateFlags::SIGNALED);
        let fence = unsafe { vk.device.create_fence(&fence_create_info, None) }
            .unwrap();

        let command_pool_create_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::TRANSIENT | CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(0);
        let command_pool = unsafe { vk.device.create_command_pool(&command_pool_create_info, None) }
            .unwrap();

        let command_buffer_allocate_info = CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let command_buffer = unsafe { vk.device.allocate_command_buffers(&command_buffer_allocate_info) }
            .unwrap()
            .swap_remove(0);

        Self {
            surface,
            swapchain,
            image_ready_semaphore,
            render_finished_semaphore,
            fence,
            command_pool,
            _command_buffer: command_buffer
        }
    }

    fn destroy(self, vk: &Vulkan) {
        unsafe {
            vk.device       .destroy_command_pool(self.command_pool, None);
            vk.device       .destroy_fence(self.fence, None);
            vk.device       .destroy_semaphore(self.render_finished_semaphore, None);
            vk.device       .destroy_semaphore(self.image_ready_semaphore, None);
            vk.ext_swapchain.destroy_swapchain(self.swapchain, None);
            vk.ext_surface  .destroy_surface(self.surface, None);
        }
    }
}

