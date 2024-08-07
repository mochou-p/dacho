// dacho/src/renderer/mod.rs

// modules
    mod buffers;
    mod commands;
    mod descriptors;
    mod devices;
    mod images;
    mod presentation;
pub mod rendering;
    mod setup;

// std
use std::collections::HashMap;

use self::buffers::VertexBuffer;

// crates
use {
    anyhow::{Context, Result},
    ash::vk,
    winit::event_loop::ActiveEventLoop
};

// mod
use {
    buffers::Buffer,
    commands::{Command, CommandBuffers, CommandPool},
    descriptors::{DescriptorPool, DescriptorSet, DescriptorSetLayout, UniformBufferObject},
    devices::{Device, PhysicalDevice},
    presentation::{Surface, Swapchain},
    rendering::{Geometry, GeometryData, Pipeline, RenderPass},
    setup::{Entry, Instance}
};

// super
use super::{
    ecs::world::Id,
    app::window::Window,
};

// debug
#[cfg(debug_assertions)]
use {
    setup::Debug,
    super::{
        app::logger::Logger,
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
    _entry:                     Entry,
    instance:                   Instance,
    #[cfg(debug_assertions)]
    debug:                      Debug,
    physical_device:            PhysicalDevice,
    device:                     Device,
    surface:                    Surface,
    render_pass:                RenderPass,
    swapchain:                  Swapchain,
    descriptor_set_layout:      DescriptorSetLayout,
    pipelines:                  HashMap<String, Pipeline>,
    geometries:                 Vec<Geometry>,
    ubo:                        Buffer,
    ubo_mapped:            *mut core::ffi::c_void,
    descriptor_pool:            DescriptorPool,
    command_pool:               CommandPool,
    descriptor_set:             DescriptorSet,
    command_buffers:            CommandBuffers,
    commands:                   Vec<Command>,
    mesh_id_commands_i:         HashMap<Id, usize>
}

impl Renderer {
    pub fn new(
        event_loop:    &ActiveEventLoop,
        window:        &Window,
        geometry_data: &mut [GeometryData]
    ) -> Result<Self> {
        #[cfg(debug_assertions)] {
            log!(info, "Creating Renderer");
            log_indent!(true);
        }

        let entry                 = Entry               ::new()?;
        let instance              = Instance            ::new(event_loop, &entry)?;
        #[cfg(debug_assertions)]
        let debug                 = Debug               ::new(&entry, &instance)?;
        let physical_device       = PhysicalDevice      ::new(&instance)?;
        let device                = Device              ::new(&instance, &physical_device)?;
        let surface               = Surface             ::new(&entry, &instance, window.raw())?;
        let render_pass           = RenderPass          ::new(&device)?;
        let swapchain             = Swapchain           ::new(&instance, &device, &surface, &physical_device, &render_pass, window.width, window.height)?;
        let descriptor_set_layout = DescriptorSetLayout ::new(&device)?;
        let command_pool          = CommandPool         ::new(&device)?;

        let mut shader_info_cache = HashMap::new();
        let mut pipelines         = HashMap::new();
        let mut geometries        = Vec::with_capacity(geometry_data.len());

        #[cfg(debug_assertions)] {
            log!(info, "Processing GeometryData");
            log_indent!(true);
        }

        geometry_data.sort_by(|g1, g2| g1.id.cmp(&g2.id));

        for data in geometry_data {
            let geometry = Geometry::new(&instance, &physical_device, &device, &command_pool, data, &mut shader_info_cache)?;

            if !pipelines.contains_key(&data.shader) {
                let shader_info = shader_info_cache.get(&data.shader)
                    .context(format!("{} not found in shader info cache", data.shader))?;

                let pipeline = Pipeline::new(&device, &descriptor_set_layout, &swapchain, &render_pass, shader_info)?;

                pipelines.insert(data.shader.clone(), pipeline);
            }

            geometries.push(geometry);
        }

        #[cfg(debug_assertions)]
        log_indent!(false);

        let (ubo, ubo_mapped) = UniformBufferObject ::new_mapped_buffer(&instance, &physical_device, &device)?;
        let descriptor_pool   = DescriptorPool      ::new(&device)?;
        let descriptor_set    = DescriptorSet       ::new(&device, &descriptor_pool, &descriptor_set_layout, &ubo)?;
        let command_buffers   = CommandBuffers      ::new(&command_pool, &swapchain, &device)?;

        // 2           -> begin render pass, bind descriptor set
        // g.len() * 3 -> for each mesh: bind vertices, bind indices, draw
        // p.len()     -> for each pipeline: bind pipeline
        let mut commands           = Vec::with_capacity(2 + geometries.len() * 3 + pipelines.len());
        let mut last_pipeline      = String::new();
        let mut first_iter         = true;
        let mut mesh_id_commands_i = HashMap::with_capacity(geometries.len());

        commands.push(Command::BeginRenderPass);

        for geometry in &geometries {
            if geometry.shader != last_pipeline {
                commands.push(Command::BindPipeline(geometry.shader.clone()));

                last_pipeline.clone_from(&geometry.shader);
            }

            if first_iter {
                commands.push(Command::BindDescriptorSets);

                first_iter = false;
            }

            mesh_id_commands_i.insert(geometry.id, commands.len());

            commands.append(&mut geometry.draw());
        }

        command_buffers.record(&device, &commands, &render_pass, &swapchain, &pipelines, &descriptor_set)?;

        #[cfg(debug_assertions)]
        log_indent!(false);

        Ok(
            Self {
                _entry: entry,
                instance,
                #[cfg(debug_assertions)]
                debug,
                physical_device,
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
                descriptor_set,
                command_buffers,
                commands,
                mesh_id_commands_i
            }
        )
    }

    #[inline]
    pub fn wait_for_device(&self) {
        self.device.wait();
    }

    pub fn update_meshes(&mut self, updated_meshes: Vec<(Id, Vec<f32>)>) -> Result<()> {
        for (mesh_id, instances) in updated_meshes {
            let g = &mut self.geometries[mesh_id as usize];

            g.instance_buffer.destroy(Some(&self.device));
            g.instance_buffer = VertexBuffer::new_buffer(&self.instance, &self.physical_device, &self.device, &self.command_pool, &instances)?;
            g.instance_count  = u32::try_from(instances.len() / 16)?; // / 16 -> temp while the only shader is the default

            let i = *self.mesh_id_commands_i.get(&g.id).context("failed to get command index from mesh id")?;

            self.commands.splice(i..=i+2, g.draw());
        }

        self.command_buffers.record(&self.device, &self.commands, &self.render_pass, &self.swapchain, &self.pipelines, &self.descriptor_set)?;

        Ok(())
    }

    pub fn redraw(&mut self, time: f32) {
        let (image_index, _) = unsafe {
            self.swapchain.loader
                .acquire_next_image(
                    *self.swapchain.raw(),
                    u64::MAX,
                    self.swapchain.images_available[self.swapchain.current_image],
                    vk::Fence::null()
                )
        }
            .expect("Acquiring next image failed");

        UniformBufferObject::update(
            self.ubo_mapped,
            time
        );

        unsafe {
            self.device.raw().wait_for_fences(
                &[self.swapchain.may_begin_drawing[self.swapchain.current_image]],
                true,
                u64::MAX
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
            log_indent!(false);
            println!("\n");
            log!(info, "Destroying Renderer");
            log_indent!(true);
        }

        self.device.wait();

        self.command_pool.destroy(Some(&self.device));

        #[cfg(debug_assertions)]
        log!(info, "Destroying Pipelines");

        for pipeline in self.pipelines.values() {
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

        for geometry in &self.geometries {
            geometry.destroy(Some(&self.device));
        }

        self.device   .destroy(None);
        self.surface  .destroy(None);
        #[cfg(debug_assertions)]
        self.debug    .destroy();
        self.instance .destroy(None);

        #[cfg(debug_assertions)]
        log_indent!(false);
    }
}

