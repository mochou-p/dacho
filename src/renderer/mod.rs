// dacho/src/renderer/mod.rs 

#[cfg(debug_assertions)]
mod debug;
mod buffer;
mod color;
mod descriptor;
mod device;
mod instance;
mod pipeline;
mod render_pass;
mod surface;
mod swapchain;
mod vertex_input;

use anyhow::{Context, Result};

use ash::vk;

use winit::{
    event_loop::EventLoop,
    window::Window
};

#[cfg(debug_assertions)]
use debug::Debug;

use {
    buffer::{Buffer, IndexBuffer, VertexBuffer},
    descriptor::{UniformBufferObject, DescriptorPool, DescriptorSet, DescriptorSetLayout},
    device::Device,
    instance::Instance,
    pipeline::Pipeline,
    render_pass::RenderPass,
    surface::Surface,
    swapchain::Swapchain,
    vertex_input::{
        instance::Instance as vi_Instance,
        vertex::Vertex     as vi_Vertex
    }
};

pub struct Renderer {
    _entry:                 ash::Entry,
    instance:               Instance,
    #[cfg(debug_assertions)]
    debug:                  Debug,
    device:                 Device,
    surface:                Surface,
    render_pass:            RenderPass,
    swapchain:              Swapchain,
    descriptor_set_layout:  DescriptorSetLayout,
    pipeline:               Pipeline,
    buffers:                Vec<Buffer>,
    ubo:                    Buffer,
    ubo_mapped:             *mut std::ffi::c_void,
    descriptor_pool:        DescriptorPool,
    command_pool:           vk::CommandPool,
    command_buffers:        Vec<vk::CommandBuffer>
}

impl Renderer {
    pub fn new(
        event_loop: &EventLoop<()>,
        window:     &Window,
        width:       u32,
        height:      u32
    ) -> Result<Self> {
        let grid_size  = 10.0;
        let grid_half  = grid_size * 0.5;
        let grid_to_uv = 2.0 / grid_size;

        let vertices = vec![
            vi_Vertex::new(-grid_half, 0.0, -grid_half, grid_to_uv),
            vi_Vertex::new( grid_half, 0.0, -grid_half, grid_to_uv),
            vi_Vertex::new( grid_half, 0.0,  grid_half, grid_to_uv),
            vi_Vertex::new(-grid_half, 0.0,  grid_half, grid_to_uv)
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
                    vi_Instance::new(
                        grid_size * (x as f32 - offset),
                        0.0,
                        grid_size * (z as f32 - offset)
                    )
                );
            }
        }

        let entry = unsafe { ash::Entry::load() }?;

        let _instance = Instance::new(
            event_loop,
            &entry
        )?;

        let instance = &_instance.instance;

        #[cfg(debug_assertions)]
        let debug = Debug::new(
            &entry,
            instance
        )?;

        let physical_device = unsafe { instance.enumerate_physical_devices() }?
            .into_iter()
            .next()
            .context("No physical devices")?;

        let _device = Device::new(
            instance,
            &physical_device
        )?;

        let device = &_device.device;
        let queue  = &_device.queue;

        let surface = Surface::new(
            &entry,
            instance,
            window
        )?;

        let render_pass = RenderPass::new(
            &device
        )?;

        let swapchain = Swapchain::new(
            instance,
            &device,
            &surface,
            &physical_device,
            &render_pass.render_pass,
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
            &render_pass.render_pass
        )?;

        let command_pool = {
            let create_info = vk::CommandPoolCreateInfo::builder()
                .queue_family_index(0)
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

            unsafe { device.create_command_pool(&create_info, None) }?
        };

        let vertex_buffer = VertexBuffer::new(
            instance,
            &physical_device,
            &device,
            &queue,
            &command_pool,
            &vertices
        )?;

        let instance_buffer = VertexBuffer::new(
            instance,
            &physical_device,
            &device,
            &queue,
            &command_pool,
            &instances
        )?;

        let index_buffer = IndexBuffer::new(
            instance,
            &physical_device,
            &device,
            &queue,
            &command_pool,
            &indices
        )?;

        let (ubo, ubo_mapped) = UniformBufferObject::new(
            instance,
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
            &ubo.buffer
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
                .render_pass(render_pass.render_pass)
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

                let vertex_buffers = [
                    vertex_buffer.buffer,
                    instance_buffer.buffer
                ];

                let offsets        = [0, 0];

                device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
                device.cmd_bind_index_buffer(command_buffer, index_buffer.buffer, 0, vk::IndexType::UINT16);

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

        let mut buffers = vec![];
        buffers.push(vertex_buffer);
        buffers.push(index_buffer);
        buffers.push(instance_buffer);

        Ok(
            Renderer {
                _entry:   entry,
                instance: _instance,
                #[cfg(debug_assertions)]
                debug,
                device: _device,
                surface,
                render_pass,
                swapchain,
                descriptor_set_layout,
                pipeline,
                buffers,
                ubo,
                ubo_mapped,
                descriptor_pool,
                command_pool,
                command_buffers
            }
        )
    }

    pub fn wait_for_device(&self) {
        self.device.wait();
    }

    pub fn redraw(
        &mut self,
        camera_transform: (glam::Vec3, glam::Vec3)
    ) {
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

        UniformBufferObject::update(
            self.ubo_mapped,
            camera_transform.0,
            camera_transform.1,
            aspect_ratio
        );

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
            }

            self.pipeline.destroy(device);
            self.render_pass.destroy(device);
            self.swapchain.destroy(device);
            self.ubo.destroy(device);
            self.descriptor_pool.destroy(device);
            self.descriptor_set_layout.destroy(device);

            for buffer in &self.buffers {
                buffer.destroy(device);
            }
        }

        self.device.destroy();
        self.surface.destroy();
        #[cfg(debug_assertions)]
        self.debug.destroy();
        self.instance.destroy();
    }
}

