// dacho/src/renderer/mod.rs 

#[cfg(debug_assertions)]
mod debug;
mod buffer;
mod color;
mod descriptor;
mod device;
mod instance;
mod pipeline;
mod surface;
mod swapchain;
mod vertex;

use anyhow::{Context, Result};

use ash::vk;

use glam::f32 as glam;

use raw_window_handle::HasRawDisplayHandle;

use winit::{
    dpi::PhysicalSize,
    event::KeyEvent,
    event_loop::EventLoop,
    keyboard::{KeyCode::*, PhysicalKey::Code},
    window::{Window, WindowBuilder}
};

#[cfg(debug_assertions)]
use debug::{Debug, messenger_create_info};

use {
    buffer::{Buffer, IndexBuffer, VertexBuffer},
    descriptor::{UniformBufferObject, DescriptorPool, DescriptorSet, DescriptorSetLayout},
    device::Device,
    instance::Instance,
    pipeline::Pipeline,
    surface::Surface,
    swapchain::Swapchain,
    vertex::Vertex
};

type MovementVector = ((f32, f32), (f32, f32), (f32, f32));

#[cfg(debug_assertions)]
const VALIDATION_LAYERS: [&'static str; 1] = [
    "VK_LAYER_KHRONOS_validation"
];

pub struct Renderer {
    _entry:                 ash::Entry,
    instance:               ash::Instance,
    #[cfg(debug_assertions)]
    debug:                  Debug,
    device:                 Device,
    window:                 Window,
    surface:                Surface,
    render_pass:            vk::RenderPass,
    swapchain:              Swapchain,
    descriptor_set_layout:  DescriptorSetLayout,
    pipeline:               Pipeline,
    vertex_buffer:          vk::Buffer,
    vertex_buffer_memory:   vk::DeviceMemory,
    instance_buffer:        vk::Buffer,
    instance_buffer_memory: vk::DeviceMemory,
    index_buffer:           vk::Buffer,
    index_buffer_memory:    vk::DeviceMemory,
    ubo:                    vk::Buffer,
    ubo_memory:             vk::DeviceMemory,
    ubo_mapped:             *mut std::ffi::c_void,
    descriptor_pool:        DescriptorPool,
    command_pool:           vk::CommandPool,
    command_buffers:        Vec<vk::CommandBuffer>,
    _start_time:            std::time::Instant,
    position:               glam::Vec3,
    movement:               MovementVector,
    direction:              glam::Vec3
}

#[cfg(debug_assertions)]
fn compile_shaders() -> Result<()> {
    let mut filepath = std::env::current_dir()?;
    filepath.push("compile_shaders.py");

    std::process::Command::new("python")
        .arg(
            filepath
                .display()
                .to_string()
        )
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

fn movement_to_vec3(m: MovementVector) -> glam::Vec3 {
    glam::Vec3::new(
        m.0.0 + m.0.1,
        m.1.0 + m.1.1,
        m.2.0 + m.2.1
    )
}

impl Renderer {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        #[cfg(debug_assertions)]
        compile_shaders()?;

        let grid_size  = 10.0;
        let grid_half  = grid_size * 0.5;
        let grid_to_uv = 2.0 / grid_size;

        let vertices = vec![
            Vertex::new(-grid_half, 0.0, -grid_half, grid_to_uv),
            Vertex::new( grid_half, 0.0, -grid_half, grid_to_uv),
            Vertex::new( grid_half, 0.0,  grid_half, grid_to_uv),
            Vertex::new(-grid_half, 0.0,  grid_half, grid_to_uv)
        ];

        let indices: Vec<u16> = vec![
            0, 1, 2,
            2, 3, 0
        ];

        let mut instances = vec![];

        let i      = 2_usize.pow(8);
        let offset = (i - 1) as f32 * 0.5;

        for z in 0..i {
            for x in 0..i {
                instances.push(
                    Instance::new(
                        grid_size * (x as f32 - offset),
                        0.0,
                        grid_size * (z as f32 - offset)
                    )
                );
            }
        }

        let entry = unsafe { ash::Entry::load() }?;

        let instance = {
            let application_info = vk::ApplicationInfo::builder()
                .api_version(vk::API_VERSION_1_3);

            let required_extensions = ash_window::enumerate_required_extensions(
                event_loop.raw_display_handle()
            )?;

            #[cfg(debug_assertions)]
            {
                let mut extension_names = required_extensions.to_vec();

                let debug     = std::ffi::CString::new("VK_EXT_debug_utils")?;
                let debug_ptr = debug.as_ptr();

                extension_names.push(debug_ptr);

                let debug_utils_create_info = messenger_create_info();

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
            }

            #[cfg(not(debug_assertions))]
            {
                let create_info = vk::InstanceCreateInfo::builder()
                    .application_info(&application_info)
                    .enabled_extension_names(required_extensions);

                unsafe { entry.create_instance(&create_info, None) }?
            }
        };

        #[cfg(debug_assertions)]
        let debug = Debug::new(
            &entry,
            &instance
        )?;

        let physical_device = unsafe { instance.enumerate_physical_devices() }?
            .into_iter()
            .next()
            .context("No physical devices")?;

        let _device = Device::new(
            &instance,
            &physical_device
        )?;

        let device = &_device.device;
        let queue  = &_device.queue;

        let scale  = 70;
        let width  = 16 * scale;
        let height = 12 * scale;

        let window = WindowBuilder::new()
            .with_title("dacho")
            .with_inner_size(PhysicalSize::new(width, height))
            .build(event_loop)?;

        window.set_cursor_grab(winit::window::CursorGrabMode::Locked)?;
        window.set_cursor_visible(false);

        let surface = Surface::new(
            &entry,
            &instance,
            &window
        )?;

        let render_pass = {
            let attachments = [
                vk::AttachmentDescription::builder()
                    .format(vk::Format::B8G8R8A8_SRGB)
                    .load_op(vk::AttachmentLoadOp::CLEAR)
                    .store_op(vk::AttachmentStoreOp::STORE)
                    .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                    .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                    .initial_layout(vk::ImageLayout::UNDEFINED)
                    .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                    .samples(vk::SampleCountFlags::TYPE_1)
                    .build(),
                vk::AttachmentDescription::builder()
                    .format(vk::Format::D32_SFLOAT_S8_UINT)
                    .load_op(vk::AttachmentLoadOp::CLEAR)
                    .store_op(vk::AttachmentStoreOp::DONT_CARE)
                    .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                    .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                    .initial_layout(vk::ImageLayout::UNDEFINED)
                    .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                    .samples(vk::SampleCountFlags::TYPE_1)
                    .build()
            ];

            let color_attachments = [
                vk::AttachmentReference::builder()
                    .attachment(0)
                    .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .build()
            ];

            let depth_attachment = vk::AttachmentReference::builder()
                .attachment(1)
                .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

            let subpasses = [
                vk::SubpassDescription::builder()
                    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                    .color_attachments(&color_attachments)
                    .depth_stencil_attachment(&depth_attachment)
                    .build()
            ];

            let subpass_dependencies = [
                vk::SubpassDependency::builder()
                    .src_subpass(vk::SUBPASS_EXTERNAL)
                    .src_stage_mask(
                        vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT |
                            vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS
                    )
                    .dst_subpass(0)
                    .dst_stage_mask(
                        vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT |
                            vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
                    .dst_access_mask(
                        vk::AccessFlags::COLOR_ATTACHMENT_WRITE |
                            vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
                    )
                    .build()
            ];

            let create_info = vk::RenderPassCreateInfo::builder()
                    .attachments(&attachments)
                    .subpasses(&subpasses)
                    .dependencies(&subpass_dependencies);

            unsafe { device.create_render_pass(&create_info, None) }?
        };

        let swapchain = Swapchain::new(
            &instance,
            &device,
            &surface,
            &physical_device,
            &render_pass,
            width,
            height
        )?;

        let descriptor_set_layout = DescriptorSetLayout::new(
            &device
        )?;

        let pipeline = Pipeline::new(
            &device,
            &descriptor_set_layout,
            &swapchain,
            &render_pass
        )?;

        let command_pool = {
            let create_info = vk::CommandPoolCreateInfo::builder()
                .queue_family_index(0)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

            unsafe { device.create_command_pool(&create_info, None) }?
        };

        let (vertex_buffer, vertex_buffer_memory) = VertexBuffer::new(
            &instance,
            &physical_device,
            &device,
            &queue,
            &command_pool,
            &vertices
        )?;

        let (instance_buffer, instance_buffer_memory) = VertexBuffer::new(
            &instance,
            &physical_device,
            &device,
            &queue,
            &command_pool,
            &instances
        )?;

        let (index_buffer, index_buffer_memory) = IndexBuffer::new(
            &instance,
            &physical_device,
            &device,
            &queue,
            &command_pool,
            &indices
        )?;

        let (ubo, ubo_memory, ubo_mapped) = UniformBufferObject::new(
            &instance,
            &physical_device,
            &device
        )?;

        let descriptor_pool = DescriptorPool::new(
            &device
        )?;

        let descriptor_set = DescriptorSet::new(
            &device,
            &descriptor_pool,
            &descriptor_set_layout,
            &ubo
        )?;

        let command_buffers = {
            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(command_pool)
                .command_buffer_count(swapchain.image_count as u32);

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
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth:   1.0,
                        stencil: 0
                    }
                }
            ];

            let begin_info = vk::RenderPassBeginInfo::builder()
                .render_pass(render_pass)
                .framebuffer(swapchain.framebuffers[i])
                .render_area(
                    vk::Rect2D::builder()
                        .offset(
                            vk::Offset2D::builder()
                                .x(0)
                                .y(0)
                                .build()
                        )
                        .extent(swapchain.extent)
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
                    pipeline.pipeline
                );

                let vertex_buffers = [vertex_buffer, instance_buffer];
                let offsets        = [0, 0];

                device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
                device.cmd_bind_index_buffer(command_buffer, index_buffer, 0, vk::IndexType::UINT16);

                let descriptor_sets = [descriptor_set];

                device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline.layout,
                    0,
                    &descriptor_sets,
                    &[]
                );

                device.cmd_draw_indexed(
                    command_buffer,
                    indices.len()   as u32,
                    instances.len() as u32,
                    0,
                    0,
                    0
                );

                device.cmd_end_render_pass(command_buffer);
                device.end_command_buffer(command_buffer)?;
            }
        }

        let _start_time = std::time::Instant::now();
        let position    = glam::Vec3::Y * 15.0;
        let movement    = ((0.0, 0.0), (0.0, 0.0), (0.0, 0.0));
        let direction   = -glam::Vec3::Z;

        Ok(
            Renderer {
                _entry: entry,
                instance,
                #[cfg(debug_assertions)]
                debug,
                device: _device,
                window,
                surface,
                render_pass,
                swapchain,
                descriptor_set_layout,
                pipeline,
                vertex_buffer,
                vertex_buffer_memory,
                instance_buffer,
                instance_buffer_memory,
                index_buffer,
                index_buffer_memory,
                ubo,
                ubo_memory,
                ubo_mapped,
                descriptor_pool,
                command_pool,
                command_buffers,
                _start_time,
                position,
                movement,
                direction
            }
        )
    }

    pub fn keyboard_input(&mut self, event: &KeyEvent) {
        if event.repeat {
            return;
        }

        static SPEED: f32 = 0.2;

        match event.physical_key {
            Code(KeyA)      => { self.movement.0.0 = -SPEED * (1.0 - event.state as i32 as f32); },
            Code(KeyD)      => { self.movement.0.1 =  SPEED * (1.0 - event.state as i32 as f32); },
            Code(KeyW)      => { self.movement.2.0 = -SPEED * (1.0 - event.state as i32 as f32); },
            Code(KeyS)      => { self.movement.2.1 =  SPEED * (1.0 - event.state as i32 as f32); },
            Code(ShiftLeft) => { self.movement.1.0 = -SPEED * (1.0 - event.state as i32 as f32); },
            Code(Space)     => { self.movement.1.1 =  SPEED * (1.0 - event.state as i32 as f32); },
            _ => ()
        }
    }

    pub fn mouse_input(&mut self, delta: &(f64, f64)) {
        static SPEED:   f32 = -0.001;
        static PHI_MIN: f32 = -std::f32::consts::PI * 0.5 + f32::EPSILON;
        static PHI_MAX: f32 =  std::f32::consts::PI * 0.5 - f32::EPSILON;

        unsafe {
            static mut THETA: f32 = std::f32::consts::PI;
            static mut PHI:   f32 = 0.0;

            THETA += delta.0 as f32 * SPEED;

            PHI = (PHI + delta.1 as f32 * SPEED).clamp(PHI_MIN, PHI_MAX);

            self.direction.x = THETA.sin() * PHI.cos();
            self.direction.y = PHI.sin();
            self.direction.z = THETA.cos() * PHI.cos();
        }
    }

    pub fn update(&mut self) {
        self.position += movement_to_vec3(self.movement);
    }

    fn _time(&self) -> f32 {
        self._start_time.elapsed().as_secs_f32()
    }

    pub fn wait_for_device(&self) {
        self.device.wait();
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn redraw(&mut self) {
        let device = &self.device.device;
        let queue  = &self.device.queue;

        let (image_index, _) = unsafe {
            self.swapchain.loader
                .acquire_next_image(
                    self.swapchain.swapchain,
                    std::u64::MAX,
                    self.swapchain.images_available[self.swapchain.current_image],
                    vk::Fence::null()
                )
        }
            .expect("Acquiring next image failed");

        let aspect_ratio = (self.swapchain.extent.width as f32) / (self.swapchain.extent.height as f32);
        UniformBufferObject::update(self.ubo_mapped, self.position, self.direction, aspect_ratio);

        unsafe {
            device.wait_for_fences(
                &[self.swapchain.may_begin_drawing[self.swapchain.current_image]],
                true,
                std::u64::MAX
            )
        }
            .expect("Waiting for fences failed");

        unsafe {
            device.reset_fences(
                &[self.swapchain.may_begin_drawing[self.swapchain.current_image]]
            )
        }
            .expect("Resetting fences failed");

        let semaphores_available = [
            self.swapchain.images_available[self.swapchain.current_image]
        ];

        let waiting_stages = [
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
        ];

        let command_buffers = [
            self.command_buffers[image_index as usize]
        ];

        let semaphores_finished = [
            self.swapchain.images_finished[self.swapchain.current_image]
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
            device.queue_submit(
                *queue,
                &submit_info,
                self.swapchain.may_begin_drawing[self.swapchain.current_image]
            )
        }
            .expect("Submitting queue failed");

        let swapchains    = [self.swapchain.swapchain];
        let image_indices = [image_index];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&semaphores_finished)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            self.swapchain.loader.queue_present(
                *queue,
                &present_info
            )
        }
            .expect("Presenting queue failed");

        self.swapchain.current_image =
            (self.swapchain.current_image + 1)
            % self.swapchain.image_count;
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.device.wait();

        {
            let device = &self.device.device;

            unsafe {
                device.destroy_command_pool(self.command_pool, None);
                self.pipeline.destroy(device);
                device.destroy_render_pass(self.render_pass, None);
            }

            self.swapchain.destroy(device);

            Buffer::destroy(device, &self.ubo, &self.ubo_memory);

            self.descriptor_pool.destroy(device);
            self.descriptor_set_layout.destroy(device);

            Buffer::destroy(device,   &self.vertex_buffer,   &self.vertex_buffer_memory);
            Buffer::destroy(device, &self.instance_buffer, &self.instance_buffer_memory);
            Buffer::destroy(device,    &self.index_buffer,    &self.index_buffer_memory);
        }

        self.device.destroy();
        self.surface.destroy();

        #[cfg(debug_assertions)]
        self.debug.destroy();

        unsafe { self.instance.destroy_instance(None); }
    }
}

