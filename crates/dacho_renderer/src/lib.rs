// dacho/crates/dacho_renderer/src/lib.rs

#![expect(
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
    reason = "most of vulkan is unsafe"
)]

use std::{any, ffi, fs, iter, mem, ptr, slice, collections::HashMap};

use ash::khr::{surface, swapchain};
use ash::vk;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub use ash;


const   SWAPCHAIN_FORMAT: vk::Format = vk::Format::R8G8B8A8_SRGB;
const        VERTEX_SIZE: usize      = 2;
const         INDEX_SIZE: usize      = 3;
const      INSTANCE_SIZE: usize      = 2;

const PUSH_CONSTANTS_LEN: usize = {
    3 * mem::size_of::<u64>()
    +
    1 * mem::size_of::<u32>()
};

type SwapchainAndEverythingRelated = (
    vk::Extent2D,
    vk::ImageSubresourceRange,
    vk::SwapchainKHR,
    Vec<vk::Image>,
    Vec<vk::ImageView>,
    [vk::Viewport; 1],
    [vk::Rect2D;   1],
    u32
);

trait Mesh {
    fn vertices() -> &'static [[f32; VERTEX_SIZE]];
    fn  indices() -> &'static [[u32;  INDEX_SIZE]];
}

struct Quad;
impl Mesh for Quad {
    fn vertices() -> &'static [[f32; VERTEX_SIZE]] {
        &[
            [-0.1, -0.1],
            [-0.1,  0.1],
            [ 0.1, -0.1],
            [ 0.1,  0.1]
        ]
    }

    fn indices() -> &'static [[u32; INDEX_SIZE]] {
        &[
            [0, 1, 2],
            [2, 1, 3]
        ]
    }
}

struct Circle;
impl Mesh for Circle {
    fn vertices() -> &'static [[f32; VERTEX_SIZE]] {
        &[
            [ 0.0, -0.10],
            [ 0.0,  0.00],
            [ 0.1, -0.05],
            [ 0.1,  0.05],
            [ 0.0,  0.10],
            [-0.1,  0.05],
            [-0.1, -0.05]
        ]
    }

    fn indices() -> &'static [[u32; INDEX_SIZE]] {
        &[
            [0, 1, 2],
            [2, 1, 3],
            [3, 1, 4],
            [4, 1, 5],
            [5, 1, 6],
            [6, 1, 0]
        ]
    }
}

struct InstanceData {
    chunk_offset: usize,
    count:        usize
}

#[derive(Default)]
struct MeshData {
    instance_count_estimate: usize,
    vertex_offset:           usize,
    index_offset:            usize,
    index_count:             usize
}

#[derive(Default)]
struct Meshes {
    registered:                    HashMap<String, MeshData>,
    instance_datas_per_mesh:       HashMap<String, Vec<InstanceData>>,
    current_vertex_offset:         usize,
    current_index_offset:          usize,
    current_instance_chunk_offset: usize,
    vertices:                      Vec<f32>,
    indices:                       Vec<u32>,
    instances:                     Vec<f32>
}

impl Meshes {
    fn with_size_estimates(
        different_meshes_count: usize,
        vertex_buffer_size:     usize,
        index_buffer_size:      usize,
        instance_buffer_size:   usize
    ) -> Self {
        Self {
            registered: HashMap::with_capacity(different_meshes_count),
            vertices:       Vec::with_capacity(    vertex_buffer_size),
            indices:        Vec::with_capacity(     index_buffer_size),
            instances:      Vec::with_capacity(  instance_buffer_size),
            ..Default::default()
        }
    }

    fn register<M: Mesh>(&mut self, instance_count_estimate: usize) {
        let key = any::type_name::<M>().to_owned();

        assert!(!self.registered.contains_key(&key), "`{key}` is already registered!");

        let     vertices = M::vertices();
        let      indices = M:: indices();
        let vertex_count = vertices.len() * VERTEX_SIZE;
        let  index_count =  indices.len() *  INDEX_SIZE;

        let mesh_data = MeshData {
            instance_count_estimate,
            vertex_offset: self.current_vertex_offset,
            index_offset:  self.current_index_offset,
            index_count
        };

        self.vertices.extend(unsafe {
            slice::from_raw_parts(vertices.as_ptr().cast::<f32>(), vertex_count)
        });
        self.indices .extend(unsafe {
            slice::from_raw_parts( indices.as_ptr().cast::<u32>(),  index_count)
        });

        self.registered.insert(key, mesh_data);
        self.current_vertex_offset += vertex_count;
        self.current_index_offset  +=  index_count;
    }

    fn add_instance<M: Mesh>(&mut self, instance: [f32; INSTANCE_SIZE]) {
        let key = any::type_name::<M>().to_owned();

        let mesh_data = self.registered.get(&key)
            .unwrap_or_else(|| panic!("`{key}` has not yet been registered!"));

        let estimated_size = mesh_data.instance_count_estimate * INSTANCE_SIZE;

        let i = {
            if let Some(instance_datas) = self.instance_datas_per_mesh.get_mut(&key) {
                let instance_data = instance_datas.last_mut().unwrap();

                if instance_data.count == mesh_data.instance_count_estimate {
                    // another chunk for M

                    let new_chunk = InstanceData {
                        chunk_offset: self.current_instance_chunk_offset,
                        count:        1
                    };
                    let i = new_chunk.chunk_offset;

                    self.instances.resize(self.instances.len() + estimated_size, 0.0);
                    instance_datas.push(new_chunk);

                    self.current_instance_chunk_offset += estimated_size;

                    i
                } else {
                    // last chunk for M

                    let i = instance_data.chunk_offset + (instance_data.count * INSTANCE_SIZE);

                    instance_data.count += 1;

                    i
                }
            } else {
                // first chunk for M

                let instance_data = InstanceData {
                    chunk_offset: self.current_instance_chunk_offset,
                    count:        1
                };
                let i = instance_data.chunk_offset;

                self.instances.resize(self.instances.len() + estimated_size, 0.0);

                self.instance_datas_per_mesh.insert(key, vec![instance_data]);

                self.current_instance_chunk_offset += estimated_size;

                i
            }
        };

        self.instances.splice(i..i + INSTANCE_SIZE, instance);
    }

    fn draw(
        &self,
        vk:             &Vulkan,
        command_buffer: vk::CommandBuffer,
        renderer:       &Renderer
    ) {
        // TODO: overwrite offseted ranges of bytes, instead of changing len
        //       easy when it will be incorporated into the type system or with proc-macros
        let mut push_constants = renderer. vertices_pointer.to_le_bytes().to_vec();
        push_constants.extend(   renderer.  indices_pointer.to_le_bytes());
        push_constants.extend(   renderer.instances_pointer.to_le_bytes());

        let cut_off1 = push_constants.len();

        // TODO: dont do `/ {VERTEX/INSTANCE}_SIZE` here
        //       rather do more work in Self::add_instance
        for (key, instance_datas) in &self.instance_datas_per_mesh {
            let mesh_data = &self.registered[key];

            push_constants.truncate(cut_off1);
            push_constants.extend((mesh_data.index_offset as u32).to_le_bytes());

            for instance_data in instance_datas {
                unsafe {
                    vk.device.cmd_push_constants(command_buffer, renderer.pipeline_layout, vk::ShaderStageFlags::VERTEX, 0, &push_constants);

                    // NOTE: its indexed inside the shader
                    vk.device.cmd_draw(
                        command_buffer,
                        mesh_data.index_count as u32,
                        instance_data.count   as u32,
                        (mesh_data.vertex_offset / VERTEX_SIZE) as u32,
                        (instance_data.chunk_offset / INSTANCE_SIZE) as u32
                    );
                }
            }
        }
    }
}

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
            .logic_op(true)
            .shader_int64(true);
        let mut vulkan13_extensions = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);
        let mut vulkan12_extensions = vk::PhysicalDeviceVulkan12Features::default()
            .buffer_device_address(true);
        let device_create_info = vk::DeviceCreateInfo::default()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(enabled_extension_names)
            .enabled_features(&enabled_features)
            .push_next(&mut vulkan13_extensions)
            .push_next(&mut vulkan12_extensions);
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
    pub fn new_renderer(
        &self,
        handle:      impl HasDisplayHandle + HasWindowHandle,
        width:       u32,
        height:      u32,
        clear_color: [f32; 4]
    ) -> Renderer {
        Renderer::new(self, handle, width, height, clear_color)
    }

    pub fn destroy_renderer(&self, renderer: Renderer) {
        renderer.destroy(self);
    }

    // TODO: make one big allocation from which smaller chunks are taken
    fn create_buffer<T>(
        &self,
        data:  &[T],
        usage: vk::BufferUsageFlags
    ) -> (vk::Buffer, vk::DeviceMemory) {
        let len                = data.len();
        let size               = mem::size_of_val(data) as u64;
        let buffer_create_info = vk::BufferCreateInfo::default()
            .size(size)
            .usage(
                usage                                |
                vk::BufferUsageFlags::STORAGE_BUFFER |
                vk::BufferUsageFlags::TRANSFER_DST   |
                vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let buffer = unsafe { self.device.create_buffer(&buffer_create_info, None) }
            .unwrap();
        let memory_requirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };
        let memory_properties   = unsafe { self.instance.get_physical_device_memory_properties(self.physical_device) };
        let memory_type_index   = find_memory_type_index(
            &memory_properties,
            memory_requirements.memory_type_bits,
            vk::MemoryPropertyFlags::DEVICE_LOCAL  |
            vk::MemoryPropertyFlags::HOST_VISIBLE  |
            vk::MemoryPropertyFlags::HOST_COHERENT
        ).unwrap();

        let mut memory_allocate_flags_info = vk::MemoryAllocateFlagsInfo::default()
            .flags(vk::MemoryAllocateFlags::DEVICE_ADDRESS);
        let memory_allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index)
            .push_next(&mut memory_allocate_flags_info);
        let device_memory = unsafe { self.device.allocate_memory(&memory_allocate_info, None) }
            .unwrap();

        unsafe { self.device.bind_buffer_memory(buffer, device_memory, 0) }
            .unwrap();

        // TODO: replace with a staging buffer
        //       or maybe not? to keep easy cpu map for frequent changes
        {
            let src = data.as_ptr();
            let dst = unsafe { self.device.map_memory(device_memory, 0, size, vk::MemoryMapFlags::empty()) }
                .unwrap()
                .cast::<T>();

            unsafe {
                ptr::copy_nonoverlapping(src, dst, len);
                self.device.unmap_memory(device_memory);
            }
        }

        (buffer, device_memory)
    }

    #[inline]
    pub fn device_wait_idle(&self) {
        unsafe { self.device.device_wait_idle() }
            .unwrap();
    }

    #[inline]
    pub fn render(
        &self,
        renderer:                 &mut Renderer,
        winit_pre_present_notify: impl Fn()
    ) {
        let fi = renderer.frame_index as usize;

        let in_flight_fence           = renderer.in_flight_fences          [fi];
        let image_ready_semaphore     = renderer.image_ready_semaphores    [fi];
        let render_finished_semaphore = renderer.render_finished_semaphores[fi];
        let command_buffer            = renderer.command_buffers           [fi];

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
                    }

                    renderer.meshes.draw(self, command_buffer, renderer);
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

        renderer.frame_index = (renderer.frame_index + 1) % renderer.max_frames_in_flight;
    }

    #[inline]
    pub fn resize(&self, renderer: &mut Renderer, width: u32, height: u32) {
        self.device_wait_idle();

        let (
            image_extent,
            subresource_range,
            swapchain,
            swapchain_images,
            swapchain_image_views,
            viewports,
            scissors,
            _
        ) = self.create_swapchain_and_everything_related(renderer.surface, width, height, renderer.swapchain);

        renderer.destroy_swapchain_and_image_views(self);

        renderer.image_extent          = image_extent;
        renderer.subresource_range     = subresource_range;
        renderer.swapchain             = swapchain;
        renderer.swapchain_images      = swapchain_images;
        renderer.swapchain_image_views = swapchain_image_views;
        renderer.viewports             = viewports;
        renderer.scissors              = scissors;
    }

    #[inline]
    fn create_swapchain_and_everything_related(
        &self,
        surface:       vk::SurfaceKHR,
        width:         u32,
        height:        u32,
        old_swapchain: vk::SwapchainKHR
    ) -> SwapchainAndEverythingRelated {
        let image_extent = vk::Extent2D { width, height };

        let subresource_range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);

        let surface_capabilities = unsafe { self.ext_surface.get_physical_device_surface_capabilities(self.physical_device, surface) }
            .unwrap();
        let max_frames_in_flight = 4; // surface_capabilities.min_image_count + 1;
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
            .old_swapchain(old_swapchain)
            .surface(surface)
            .image_format(SWAPCHAIN_FORMAT)
            .image_extent(image_extent)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_array_layers(1)
            .min_image_count(max_frames_in_flight)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .pre_transform(surface_capabilities.current_transform)
            .clipped(true)
            .present_mode(vk::PresentModeKHR::FIFO);
        let swapchain = unsafe { self.ext_swapchain.create_swapchain(&swapchain_create_info, None) }
            .unwrap();

        let swapchain_images = unsafe { self.ext_swapchain.get_swapchain_images(swapchain) }
            .unwrap();

        assert!(swapchain_images.len() == max_frames_in_flight as usize, "swapchain image count error");

        let swapchain_image_views = swapchain_images
            .iter()
            .map(|image| {
                let image_view_create_info = vk::ImageViewCreateInfo::default()
                    .image(*image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(SWAPCHAIN_FORMAT)
                    .subresource_range(subresource_range);

                let image_view = unsafe { self.device.create_image_view(&image_view_create_info, None) }
                    .unwrap();

                image_view
            })
            .collect();

        let viewports = [
            vk::Viewport {
                x:         0.0,          y:         0.0,
                width:     width as f32, height:    height as f32,
                min_depth: 0.0,          max_depth: 1.0
            }
        ];
        let scissors = [image_extent.into()];

        (
            image_extent,
            subresource_range,
            swapchain,
            swapchain_images,
            swapchain_image_views,
            viewports,
            scissors,
            max_frames_in_flight
        )
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
    fn acquire_next_image(
        &self,
        swapchain:             vk::SwapchainKHR,
        image_ready_semaphore: vk::Semaphore
    ) -> u32 {
        let (image_index, _) = unsafe { self.ext_swapchain.acquire_next_image(swapchain, u64::MAX, image_ready_semaphore, vk::Fence::null()) }
            .unwrap();

        image_index
    }

    #[inline]
    fn with_command_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        closure:        impl Fn()
    ) {
        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe { self.device.begin_command_buffer(command_buffer, &begin_info) }
            .unwrap();

        closure();

        unsafe { self.device.end_command_buffer(command_buffer) }
            .unwrap();
    }

    #[inline]
    fn with_image_memory_barriers(
        &self,
        image:          vk::Image,
        renderer:       &Renderer,
        command_buffer: vk::CommandBuffer,
        closure:        impl Fn()
    ) {
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

        closure();

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
    fn with_rendering(
        &self,
        renderer:       &Renderer,
        image_index:    u32,
        command_buffer: vk::CommandBuffer,
        closure:        impl Fn()
    ) {
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

        closure();

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
    subresource_range:          vk::ImageSubresourceRange,
    swapchain_images:           Vec<vk::Image>,
    swapchain_image_views:      Vec<vk::ImageView>,
    image_ready_semaphores:     Vec<vk::Semaphore>,
    render_finished_semaphores: Vec<vk::Semaphore>,
    in_flight_fences:           Vec<vk::Fence>,
    command_pool:               vk::CommandPool,
    command_buffers:            Vec<vk::CommandBuffer>,
    viewports:                  [vk::Viewport; 1],
    scissors:                   [vk::Rect2D;   1],
    pipeline_layout:            vk::PipelineLayout,
    pipeline:                   vk::Pipeline,
    clear_value:                vk::ClearValue,
    frame_index:                u32,
    max_frames_in_flight:       u32,
    vertices:                   (vk::Buffer, vk::DeviceMemory),
    indices:                    (vk::Buffer, vk::DeviceMemory),
    instances:                  (vk::Buffer, vk::DeviceMemory),
    meshes:                     Meshes,
    vertices_pointer:           u64,
    indices_pointer:            u64,
    instances_pointer:          u64
}

impl Renderer {
    #[must_use]
    fn new(
        vk:          &Vulkan,
        handle:      impl HasDisplayHandle + HasWindowHandle,
        width:       u32,
        height:      u32,
        clear_color: [f32; 4]
    ) -> Self {
        let rdh = handle
            .display_handle()
            .unwrap()
            .into();
        let rwh = handle
            .window_handle()
            .unwrap()
            .into();
        let surface = unsafe { ash_window::create_surface(&vk.entry, &vk.instance, rdh, rwh, None) }
            .unwrap();

        let (
            image_extent,
            subresource_range,
            swapchain,
            swapchain_images,
            swapchain_image_views,
            viewports,
            scissors,
            max_frames_in_flight
        ) = vk.create_swapchain_and_everything_related(surface, width, height, vk::SwapchainKHR::null());

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();
        let image_ready_semaphores = iter::repeat_with(|| {
            unsafe { vk.device.create_semaphore(&semaphore_create_info, None) }
                .unwrap()
        }).take(max_frames_in_flight as usize).collect();
        let render_finished_semaphores = iter::repeat_with(|| {
            unsafe { vk.device.create_semaphore(&semaphore_create_info, None) }
                .unwrap()
        }).take(max_frames_in_flight as usize).collect();

        let fence_create_info = vk::FenceCreateInfo::default()
            .flags(vk::FenceCreateFlags::SIGNALED);
        let in_flight_fences = iter::repeat_with(|| {
            unsafe { vk.device.create_fence(&fence_create_info, None) }
                .unwrap()
        }).take(max_frames_in_flight as usize).collect();

        let command_pool_create_info = vk::CommandPoolCreateInfo::default()
            .flags(vk::CommandPoolCreateFlags::TRANSIENT | vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(0);
        let command_pool = unsafe { vk.device.create_command_pool(&command_pool_create_info, None) }
            .unwrap();

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(max_frames_in_flight);
        let command_buffers = unsafe { vk.device.allocate_command_buffers(&command_buffer_allocate_info) }
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
            .primitive_restart_enable(false)
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);
        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
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
        let push_constant_ranges = [
            vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::VERTEX)
                .offset(0)
                .size(u32::try_from(PUSH_CONSTANTS_LEN).unwrap())
        ];
        let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo::default()
            .push_constant_ranges(&push_constant_ranges);
        let pipeline_layout = unsafe { vk.device.create_pipeline_layout(&pipeline_layout_create_info, None) }
            .unwrap();
        let mut rendering_info = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&[SWAPCHAIN_FORMAT]);
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

        let clear_value = vk::ClearValue { color: vk::ClearColorValue {
            float32: clear_color
        } };

        let frame_index = 0;

        let mut meshes = Meshes::with_size_estimates(2, 32, 32, 128);

        meshes.register::<Quad>  (4);
        meshes.register::<Circle>(4);

        meshes.add_instance::<Quad>  ([ 0.0, -0.5]);
        meshes.add_instance::<Quad>  ([ 0.0,  0.5]);
        meshes.add_instance::<Quad>  ([ 0.5, -0.5]);
        meshes.add_instance::<Quad>  ([ 0.5,  0.5]);
        meshes.add_instance::<Circle>([-0.5,  0.0]);
        meshes.add_instance::<Circle>([-0.5, -0.5]);
        meshes.add_instance::<Circle>([-0.5,  0.5]);
        meshes.add_instance::<Circle>([ 0.5,  0.0]);
        meshes.add_instance::<Circle>([ 0.5,  0.0]);

        let vertices = vk.create_buffer(
            &meshes.vertices,
            vk::BufferUsageFlags::VERTEX_BUFFER
        );
        let indices = vk.create_buffer(
            &meshes.indices,
            vk::BufferUsageFlags::INDEX_BUFFER
        );
        let instances = vk.create_buffer(
            &meshes.instances,
            vk::BufferUsageFlags::VERTEX_BUFFER
        );

        let vertices_pointer = {
            let buffer_device_address_info = vk::BufferDeviceAddressInfo::default()
                .buffer(vertices.0);

            unsafe { vk.device.get_buffer_device_address(&buffer_device_address_info) }
        };
        let indices_pointer = {
            let buffer_device_address_info = vk::BufferDeviceAddressInfo::default()
                .buffer(indices.0);

            unsafe { vk.device.get_buffer_device_address(&buffer_device_address_info) }
        };
        let instances_pointer = {
            let buffer_device_address_info = vk::BufferDeviceAddressInfo::default()
                .buffer(instances.0);

            unsafe { vk.device.get_buffer_device_address(&buffer_device_address_info) }
        };

        Self {
            surface,
            image_extent,
            swapchain,
            subresource_range,
            swapchain_images,
            swapchain_image_views,
            image_ready_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            command_pool,
            command_buffers,
            viewports,
            scissors,
            pipeline_layout,
            pipeline,
            clear_value,
            frame_index,
            max_frames_in_flight,
            vertices,
            indices,
            instances,
            meshes,
            vertices_pointer,
            indices_pointer,
            instances_pointer
        }
    }

    fn destroy_swapchain_and_image_views(&mut self, vk: &Vulkan) {
        unsafe {
            self.swapchain_image_views
                .iter()
                .for_each(|image_view| vk.device.destroy_image_view(*image_view, None));

            vk.ext_swapchain.destroy_swapchain(self.swapchain, None);
        }
    }

    fn destroy(mut self, vk: &Vulkan) {
        unsafe {
            vk.device.free_memory(self.instances.1, None);
            vk.device.free_memory(self.indices.1, None);
            vk.device.free_memory(self.vertices.1, None);
            vk.device.destroy_buffer(self.instances.0, None);
            vk.device.destroy_buffer(self.indices.0, None);
            vk.device.destroy_buffer(self.vertices.0, None);
            vk.device.destroy_pipeline(self.pipeline, None);
            vk.device.destroy_pipeline_layout(self.pipeline_layout, None);
            vk.device.destroy_command_pool(self.command_pool, None);

            self.in_flight_fences
                .iter()
                .for_each(|fence| vk.device.destroy_fence(*fence, None));

            self.render_finished_semaphores
                .iter()
                .for_each(|semaphore| vk.device.destroy_semaphore(*semaphore, None));
            self.image_ready_semaphores
                .iter()
                .for_each(|semaphore| vk.device.destroy_semaphore(*semaphore, None));

            self.destroy_swapchain_and_image_views(vk);

            vk.ext_surface.destroy_surface(self.surface, None);
        }
    }
}

fn read_spirv(filepath: &str) -> Vec<u32> {
    let bytes = fs::read(format!("{filepath}.spv")).unwrap();

    assert!(!bytes.is_empty(),      "invalid SPIR-V file (empty file)");
    assert!((bytes.len() % 4) == 0, "invalid SPIR-V file (byte count is not divisible by 4)");

    let mut words = Vec::with_capacity(bytes.len() / 4);
    for chunk in bytes.chunks(4) {
        let mut word = [0_u8; 4];
        word.copy_from_slice(chunk);
        words.push(u32::from_ne_bytes(word));
    }

    assert!(words[0] == 0x0723_0203, "invalid SPIR-V file (first word is not SPIR-V magic number)");

    words
}

fn find_memory_type_index(
    memory_properties:   &vk::PhysicalDeviceMemoryProperties,
    memory_type_bits:    u32,
    required_properties: vk::MemoryPropertyFlags
) -> Option<u32> {
    for i in 0..memory_properties.memory_type_count {
        let contains = memory_properties
            .memory_types[i as usize]
            .property_flags
            .contains(required_properties);

        if contains && (memory_type_bits & (1 << i)) != 0 {
            return Some(i);
        }
    }

    None
}

