// dacho/src/renderer/mod.rs 

#[cfg(debug_assertions)]
mod debug;
mod buffer;
mod color;
mod command;
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
    command::{Command, CommandBuffers, CommandPool},
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
    command_pool:           CommandPool,
    command_buffers:        CommandBuffers
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

        let instance = Instance::new(
            event_loop,
            &entry
        )?;

        #[cfg(debug_assertions)]
        let debug = Debug::new(
            &entry,
            &instance.instance
        )?;

        let physical_device = unsafe { instance.instance.enumerate_physical_devices() }?
            .into_iter()
            .next()
            .context("No physical devices")?;

        let device = Device::new(
            &instance.instance,
            &physical_device
        )?;

        let surface = Surface::new(
            &entry,
            &instance.instance,
            window
        )?;

        let render_pass = RenderPass::new(
            &device.device
        )?;

        let swapchain = Swapchain::new(
            &instance.instance,
            &device.device,
            &surface,
            &physical_device,
            &render_pass.render_pass,
            width,
            height
        )?;

        let descriptor_set_layout = DescriptorSetLayout::new(
            &device.device
        )?;

        let pipeline = Pipeline::new(
            &device.device,
            &descriptor_set_layout,
            &swapchain,
            &render_pass.render_pass
        )?;

        let command_pool = CommandPool::new(
            &device.device
        )?;

        let vertex_buffer = VertexBuffer::new(
            &instance.instance,
            &physical_device,
            &device.device,
            &device.queue,
            &command_pool.command_pool,
            &vertices
        )?;

        let instance_buffer = VertexBuffer::new(
            &instance.instance,
            &physical_device,
            &device.device,
            &device.queue,
            &command_pool.command_pool,
            &instances
        )?;

        let index_buffer = IndexBuffer::new(
            &instance.instance,
            &physical_device,
            &device.device,
            &device.queue,
            &command_pool.command_pool,
            &indices
        )?;

        let (ubo, ubo_mapped) = UniformBufferObject::new(
            &instance.instance,
            &physical_device,
            &device.device
        )?;

        let descriptor_pool = DescriptorPool::new(
            &device.device
        )?;

        let descriptor_set = DescriptorSet::new(
            &device.device,
            &descriptor_pool,
            &descriptor_set_layout,
            &ubo.buffer
        )?;

        let command_buffers = CommandBuffers::new(
            &command_pool.command_pool,
            &swapchain,
            &device.device
        )?;

        command_buffers.record(
            &device.device,
            &[
                Command::BeginRenderPass(&render_pass, &swapchain),
                Command::BindPipeline(&pipeline),
                Command::BindVertexBuffers(&vertex_buffer, &instance_buffer),
                Command::BindIndexBuffer(&index_buffer),
                Command::BindDescriptorSets(&descriptor_set),
                Command::DrawIndexed(indices.len(), instances.len())
            ]
        )?;

        let mut buffers = vec![];
        buffers.push(vertex_buffer);
        buffers.push(index_buffer);
        buffers.push(instance_buffer);

        Ok(
            Renderer {
                _entry: entry,
                instance,
                #[cfg(debug_assertions)]
                debug,
                device,
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
            self.command_buffers.command_buffers[image_index as usize]
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

            self.command_pool.destroy(device);            
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

