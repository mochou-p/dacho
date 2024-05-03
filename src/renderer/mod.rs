// dacho/src/renderer/mod.rs 

    mod buffer;
    mod command;
#[cfg(debug_assertions)]
    mod debug;
    mod descriptor;
    mod device;
pub mod geometry;
    mod image;
    mod instance;
    mod pipeline;
    mod render_pass;
    mod surface;
    mod swapchain;
    mod vertex_input;

use {
    std::collections::HashMap,
    anyhow::{Context, Result},
    ash::vk,
    winit::{
        event_loop::EventLoop,
        window::Window
    }
};

use {
    buffer::Buffer,
    command::{Command, CommandBuffers, CommandPool},
    descriptor::{UniformBufferObject, DescriptorPool, DescriptorSet, DescriptorSetLayout},
    device::{Device, PhysicalDevice},
    geometry::{Geometry, GeometryData},
    image::{Image, ImageView, Sampler, Texture, TextureView},
    instance::Instance,
    pipeline::Pipeline,
    render_pass::RenderPass,
    surface::Surface,
    swapchain::Swapchain,
};

#[cfg(debug_assertions)]
use {
    debug::Debug,
    crate::application::logger::Logger
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
    pipelines:              HashMap<String, Pipeline>,
    geometries:             Vec<Geometry>,
    ubo:                    Buffer,
    ubo_mapped:            *mut std::ffi::c_void,
    descriptor_pool:        DescriptorPool,
    command_pool:           CommandPool,
    command_buffers:        CommandBuffers,
    images:                 Vec<Image>,
    image_views:            Vec<ImageView>,
    samplers:               Vec<Sampler>
}

impl Renderer {
    pub fn new(
        event_loop:     &EventLoop<()>,
        window:         &Window,
        window_width:    u32,
        window_height:   u32,
        scene:          &[GeometryData],
        skybox_texture: &[u8],
        gltf_textures:  &[Vec<u8>]
    ) -> Result<Self> {
        #[cfg(debug_assertions)] {
            Logger::info("Creating Renderer");
            Logger::indent(1);
        }

        #[cfg(debug_assertions)]
        Logger::info("Creating Entry");
        let entry = unsafe { ash::Entry::load() }?;

        let instance        = Instance::new(event_loop, &entry)?;
        #[cfg(debug_assertions)]
        let debug           = Debug::new(&entry, &instance)?;
        let physical_device = PhysicalDevice::new(&instance)?;
        let device          = Device::new(&instance, &physical_device)?;
        let surface         = Surface::new(&entry, &instance, window)?;
        let render_pass     = RenderPass::new(&device)?;

        let swapchain = Swapchain::new(
            &instance,
            &device,
            &surface,
            &physical_device,
            &render_pass,
            window_width,
            window_height
        )?;

        let gltf_texture_count    = gltf_textures.len();
        let descriptor_set_layout = DescriptorSetLayout::new(&device, gltf_texture_count)?;
        let command_pool          = CommandPool::new(&device)?;

        let mut images      = vec![];
        let mut image_views = vec![];
        let mut samplers    = vec![];

        {
            let skybox = Texture::new_image(
                &instance, &physical_device, &device, &command_pool, skybox_texture, true
            )?;

            let skybox_view = TextureView::new_image_view(&device, &skybox)?;
            let sampler     = Sampler::new(&device, true)?;

            images.push(skybox);
            image_views.push(skybox_view);
            samplers.push(sampler);
        }

        for gltf_texture in gltf_textures.iter() {
            let texture = Texture::new_image(
                &instance, &physical_device, &device, &command_pool, gltf_texture, false
            )?;

            let texture_view = TextureView::new_image_view(&device, &texture)?;
            let sampler      = Sampler::new(&device, false)?;

            images.push(texture);
            image_views.push(texture_view);
            samplers.push(sampler);
        }

        let mut shader_info_cache = HashMap::new();
        let mut pipelines         = HashMap::new();
        let mut geometries        = vec![];

        #[cfg(debug_assertions)] {
            Logger::info("Processing GeometryData");
            Logger::indent(1);
        }

        for geometry_data in scene.iter() {
            let geometry = Geometry::new(
                &instance,
                &physical_device,
                &device,
                &command_pool,
                geometry_data,
                &mut shader_info_cache
            )?;

            if pipelines.get(&geometry_data.shader).is_none() {
                let shader_info = shader_info_cache.get(&geometry_data.shader)
                    .context(format!("{} not found in shader info cache", geometry_data.shader))?;

                let pipeline = Pipeline::new(
                    &device,
                    &descriptor_set_layout,
                    &swapchain,
                    &render_pass,
                    shader_info
                )?;

                pipelines.insert(geometry_data.shader.clone(), pipeline);
            }

            geometries.push(geometry);
        }

        #[cfg(debug_assertions)]
        Logger::indent(-1);

        let (ubo, ubo_mapped) = UniformBufferObject::new_mapped_buffer(&instance, &physical_device, &device)?;
        let descriptor_pool   = DescriptorPool::new(&device, gltf_texture_count)?;

        let descriptor_set = DescriptorSet::new(
            &device, &descriptor_pool, &descriptor_set_layout,
            &ubo,
            &image_views[0  ], &samplers[0  ],
            &image_views[1..], &samplers[1..]
        )?;

        let command_buffers = CommandBuffers::new(&command_pool, &swapchain, &device)?;

        let mut commands = vec![
            Command::BeginRenderPass(&render_pass, &swapchain)
        ];

        let mut last_pipeline = String::from("");
        let mut first_iter    = true;

        #[cfg(debug_assertions)]
        Logger::info("Sorting Geometry");
        geometries.sort_by(|g1, g2| g1.shader.cmp(&g2.shader));

        for geometry in geometries.iter() {
            if geometry.shader != last_pipeline {
                commands.push(
                    Command::BindPipeline(
                        pipelines.get(&geometry.shader)
                            .context("Pipeline not in hash map")?
                    )
                );

                last_pipeline = geometry.shader.clone();
            }

            if first_iter {
                commands.push(Command::BindDescriptorSets(&descriptor_set));

                first_iter = false;
            }

            commands.append(&mut geometry.draw());
        }

        command_buffers.record(&device, &commands)?;

        #[cfg(debug_assertions)]
        Logger::indent(-1);

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
                command_buffers,
                images,
                image_views,
                samplers
            }
        )
    }

    pub fn wait_for_device(&self) {
        self.device.wait();
    }

    pub fn redraw(
        &mut self,
        camera_transform: (glam::Vec3, glam::Vec3),
        time:             f32
    ) {
        let (image_index, _) = unsafe {
            self.swapchain.loader
                .acquire_next_image(
                    self.swapchain.raw,
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
            time,
            aspect_ratio
        );

        unsafe {
            self.device.raw.wait_for_fences(
                &[self.swapchain.may_begin_drawing[self.swapchain.current_image]],
                true,
                std::u64::MAX
            )
        }
            .expect("Waiting for fences failed");

        unsafe {
            self.device.raw.reset_fences(
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
            self.command_buffers.raw[image_index as usize]
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
            self.device.raw.queue_submit(
                self.device.queue,
                &submit_info,
                self.swapchain.may_begin_drawing[self.swapchain.current_image]
            )
        }
            .expect("Submitting queue failed");

        let swapchains    = [self.swapchain.raw];
        let image_indices = [image_index];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&semaphores_finished)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            self.swapchain.loader.queue_present(
                self.device.queue,
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
        #[cfg(debug_assertions)] {
            Logger::indent(-1);
            println!("\n");
            Logger::info("Destroying Renderer");
            Logger::indent(1);
        }

        self.device.wait();

        self.command_pool.destroy(&self.device);

        #[cfg(debug_assertions)]
        Logger::info("Destroying Pipelines");

        for (_, pipeline) in self.pipelines.iter() {
            pipeline.destroy(&self.device);
        }

        self.render_pass  .destroy(&self.device);
        self.swapchain    .destroy(&self.device);

        #[cfg(debug_assertions)]
        Logger::info("Destroying Textures and Samplers");

        for sampler in self.samplers.iter() {
            sampler.destroy(&self.device);
        }

        for image_view in self.image_views.iter() {
            image_view.destroy(&self.device);
        }

        for image in self.images.iter() {
            image.destroy(&self.device);
        }

        #[cfg(debug_assertions)]
        Logger::info("Destroying UniformBuffer");
        self.ubo                   .destroy(&self.device);
        self.descriptor_pool       .destroy(&self.device);
        self.descriptor_set_layout .destroy(&self.device);

        #[cfg(debug_assertions)]
        Logger::info("Destroying VertexBuffers and IndexBuffers");

        for geometry in self.geometries.iter() {
            geometry.destroy(&self.device);
        }

        self.device   .destroy();
        self.surface  .destroy();
        #[cfg(debug_assertions)]
        self.debug    .destroy();
        self.instance .destroy();

        #[cfg(debug_assertions)] {
            Logger::indent(-1);
            println!();
        }
    }
}

