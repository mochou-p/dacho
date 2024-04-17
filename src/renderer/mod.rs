// dacho/src/renderer/mod.rs 

#[cfg(debug_assertions)]
mod debug;
mod buffer;
mod command;
mod descriptor;
mod device;
mod geometry;
mod instance;
mod pipeline;
mod render_pass;
mod surface;
mod swapchain;
mod vertex_input;

use anyhow::Result;

use ash::vk;

use winit::{
    event_loop::EventLoop,
    window::Window
};

#[cfg(debug_assertions)]
use debug::Debug;

use {
    buffer::Buffer,
    command::{Command, CommandBuffers, CommandPool},
    descriptor::{UniformBufferObject, DescriptorPool, DescriptorSet, DescriptorSetLayout},
    device::{Device, PhysicalDevice},
    geometry::{Geometry, GeometryData},
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
    pipelines:              Vec<Pipeline>,
    geometries:             Vec<Geometry>,
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
        let geometries_data = vec![
            {
                let grid_size  = 16.0;
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

                let i      = 2_usize.pow(6) - 1;
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

                let pipeline_id       = Some(0);
                let descriptor_set_id = Some(0);

                GeometryData::new(
                    pipeline_id,
                    descriptor_set_id,
                    vertices,
                    instances,
                    indices
                )
            },
            {
                let w = 0.0;

                let vertices = vec![
                    vi_Vertex::new( 0.00, 4.0, 0.0, w),
                    vi_Vertex::new( 0.08, 2.4, 0.0, w),
                    vi_Vertex::new( 0.18, 0.0, 0.0, w),
                    vi_Vertex::new(-0.18, 0.0, 0.0, w),
                    vi_Vertex::new(-0.08, 1.8, 0.0, w),
                ];

                let indices = vec![
                    0, 1, 4,
                    1, 2, 3,
                    1, 3, 4
                ];

                let mut instances = vec![];

                let grid_size = 16.0;
                let i         = 32;
                let offset1   = grid_size / i as f32;
                let offset2   = (i - 1) as f32 * 0.5;

                for z in 0..i {
                    for x in 0..i {
                        instances.push(
                            vi_Instance::new(
                                offset1 * (x as f32 - offset2),
                                0.0,
                                offset1 * (z as f32 - offset2)
                            )
                        );
                    }
                }

                let pipeline_id       = Some(1);
                let descriptor_set_id = None;

                GeometryData::new(
                    pipeline_id,
                    descriptor_set_id,
                    vertices,
                    instances,
                    indices
                )
            }
        ];

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

        let physical_device = PhysicalDevice::new(
            &instance.instance
        )?;

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

        let pipelines = vec![
            Pipeline::new(
                &device.device,
                &descriptor_set_layout,
                &swapchain,
                &render_pass.render_pass,
                "tile",
                vk::CullModeFlags::BACK
            )?,
            Pipeline::new(
                &device.device,
                &descriptor_set_layout,
                &swapchain,
                &render_pass.render_pass,
                "grass",
                vk::CullModeFlags::NONE
            )?
        ];

        let command_pool = CommandPool::new(
            &device.device
        )?;

        let mut geometries = vec![];

        for geometry_data in geometries_data.iter() {
            geometries.push(
                Geometry::new(
                    &instance.instance,
                    &physical_device,
                    &device.device,
                    &device.queue,
                    &command_pool.command_pool,
                    &geometry_data
                )?
            );
        }

        let (ubo, ubo_mapped) = UniformBufferObject::new(
            &instance.instance,
            &physical_device,
            &device.device
        )?;

        let descriptor_pool = DescriptorPool::new(
            &device.device
        )?;

        let descriptor_sets = vec![
            DescriptorSet::new(
                &device.device,
                &descriptor_pool,
                &descriptor_set_layout,
                &ubo.buffer
            )?
        ];

        let command_buffers = CommandBuffers::new(
            &command_pool.command_pool,
            &swapchain,
            &device.device
        )?;

        let mut commands = vec![
            Command::BeginRenderPass(&render_pass, &swapchain)
        ];

        for geometry in geometries.iter() {
            if let Some(i) = geometry.pipeline_id {
                commands.push(Command::BindPipeline(&pipelines[i]));
            }

            if let Some(i) = geometry.descriptor_set_id {
                commands.push(Command::BindDescriptorSets(&descriptor_sets[i]));
            }

            commands.append(&mut geometry.draw());
        }

        command_buffers.record(
            &device.device,
            &commands
        )?;

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
                pipelines,
                geometries,
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

            for pipeline in self.pipelines.iter() {
                pipeline.destroy(device);
            }

            self.render_pass.destroy(device);
            self.swapchain.destroy(device);
            self.ubo.destroy(device);
            self.descriptor_pool.destroy(device);
            self.descriptor_set_layout.destroy(device);

            for geometry in self.geometries.iter() {
                geometry.destroy(device);
            }
        }

        self.device.destroy();
        self.surface.destroy();
        #[cfg(debug_assertions)]
        self.debug.destroy();
        self.instance.destroy();
    }
}

