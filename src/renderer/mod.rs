// dacho/src/renderer/mod.rs

// modules
           mod buffers;
           mod commands;
           mod descriptors;
           mod devices;
           mod images;
           mod presentation;
pub(super) mod rendering;
           mod setup;

// std
use std::collections::HashMap;

// crates
use {
    anyhow::{Context, Result},
    ash::vk,
    winit::{event_loop::EventLoop, window::Window}
};

// mod
use {
    buffers::*,
    commands::{buffers::*, pool::*, *},
    descriptors::{pool::*, set::*, set_layout::*, uniform::*},
    devices::{logical::*, physical::*},
    presentation::{surface::*, swapchain::*},
    rendering::{geometry::*, pipeline::*, render_pass::*},
    setup::{entry::*, instance::*}
};

// super
use super::application::scene::Data;

// debug
#[cfg(debug_assertions)]
use {
    setup::debug::*,
    super::{
        application::logger::Logger,
        log, log_indent
    }
};

pub trait VulkanObject {
    type RawType;

    fn raw(&self) -> &Self::RawType;

    // &Device for objects made by device
    //  None   for objects made by entry/khr loader
    fn destroy(&self, _device: Option<&Device>) {
        // empty implementation for structs that do not
        // call any .create_*()
        //
        // close vulkan wrappers implement this function,
        // while abstract dacho structs do not
    }
}

pub struct Renderer {
    _entry:                 Entry,
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
    command_buffers:        CommandBuffers
}

impl Renderer {
    pub fn new(
        event_loop:    &EventLoop<()>,
        window:        &Window,
        window_width:   u32,
        window_height:  u32,
        data:          &Data
    ) -> Result<Self> {
        #[cfg(debug_assertions)] {
            log!(info, "Creating Renderer");
            log_indent!(1);
        }

        let entry           = Entry::new()?;
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

        let descriptor_set_layout = DescriptorSetLayout::new(&device)?;
        let command_pool          = CommandPool::new(&device)?;

        let mut shader_info_cache = HashMap::new();
        let mut pipelines         = HashMap::new();
        let mut geometries        = Vec::with_capacity(data.geometry.len());

        #[cfg(debug_assertions)] {
            log!(info, "Processing GeometryData");
            log_indent!(1);
        }

        for geometry_data in data.geometry.iter() {
            let geometry = Geometry::new(
                &instance,
                &physical_device,
                &device,
                &command_pool,
                geometry_data,
                &mut shader_info_cache
            )?;

            if !pipelines.contains_key(&geometry_data.shader) {
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
        log_indent!(-1);

        let (ubo, ubo_mapped) = UniformBufferObject::new_mapped_buffer(&instance, &physical_device, &device)?;
        let descriptor_pool   = DescriptorPool::new(&device)?;
        let descriptor_set    = DescriptorSet::new(&device, &descriptor_pool, &descriptor_set_layout, &ubo)?;
        let command_buffers   = CommandBuffers::new(&command_pool, &swapchain, &device)?;

        let mut commands      = vec![Command::BeginRenderPass(&render_pass, &swapchain)];
        let mut last_pipeline = "".to_string();
        let mut first_iter    = true;

        #[cfg(debug_assertions)]
        log!(info, "Sorting Geometry");

        geometries.sort_by(|g1, g2| g1.shader.cmp(&g2.shader));

        for geometry in geometries.iter() {
            if geometry.shader != last_pipeline {
                commands.push(
                    Command::BindPipeline(
                        pipelines.get(&geometry.shader)
                            .context("Pipeline not in hash map")?
                    )
                );

                last_pipeline.clone_from(&geometry.shader);
            }

            if first_iter {
                commands.push(Command::BindDescriptorSets(&descriptor_set));

                first_iter = false;
            }

            commands.append(&mut geometry.draw());
        }

        command_buffers.record(&device, &commands)?;

        #[cfg(debug_assertions)]
        log_indent!(-1);

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

    #[inline]
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
                    *self.swapchain.raw(),
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
            self.device.raw().wait_for_fences(
                &[self.swapchain.may_begin_drawing[self.swapchain.current_image]],
                true,
                std::u64::MAX
            )
        }
            .expect("Waiting for fences failed");

        unsafe {
            self.device.raw().reset_fences(
                &[self.swapchain.may_begin_drawing[self.swapchain.current_image]]
            )
        }
            .expect("Resetting fences failed");

        let semaphores_available = [self.swapchain.images_available[self.swapchain.current_image]];
        let waiting_stages       = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT              ];
        let command_buffers      = [self.command_buffers.raw()[image_index as usize]             ];
        let semaphores_finished  = [self.swapchain.images_finished[self.swapchain.current_image] ];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&semaphores_available)
            .wait_dst_stage_mask(&waiting_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&semaphores_finished);

        unsafe {
            self.device.raw().queue_submit(
                self.device.queue,
                &[*submit_info],
                self.swapchain.may_begin_drawing[self.swapchain.current_image]
            )
        }
            .expect("Submitting queue failed");

        let swapchains    = [*self.swapchain.raw()];
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
            log_indent!(-1);
            println!("\n");
            log!(info, "Destroying Renderer");
            log_indent!(1);
        }

        self.device.wait();

        self.command_pool.destroy(Some(&self.device));

        #[cfg(debug_assertions)]
        log!(info, "Destroying Pipelines");

        for (_, pipeline) in self.pipelines.iter() {
            pipeline.destroy(Some(&self.device));
        }

        self.render_pass .destroy(Some(&self.device));
        self.swapchain   .destroy(Some(&self.device));

        #[cfg(debug_assertions)]
        log!(info, "Destroying Textures and Samplers");

        #[cfg(debug_assertions)]
        log!(info, "Destroying UniformBuffer");

        self.ubo                   .destroy(Some(&self.device));
        self.descriptor_pool       .destroy(Some(&self.device));
        self.descriptor_set_layout .destroy(Some(&self.device));

        #[cfg(debug_assertions)]
        log!(info, "Destroying VertexBuffers and IndexBuffers");

        for geometry in self.geometries.iter() {
            geometry.destroy(Some(&self.device));
        }

        self.device   .destroy(None);
        self.surface  .destroy(None);
        #[cfg(debug_assertions)]
        self.debug    .destroy();
        self.instance .destroy(None);

        #[cfg(debug_assertions)]
        log_indent!(-1);
    }
}

