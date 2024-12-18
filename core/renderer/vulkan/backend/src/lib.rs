// dacho/core/renderer/vulkan/backend/src/lib.rs

#![expect(clippy::undocumented_unsafe_blocks, reason = "vulkan is C, so ash (the rust binding) is unsafe")]

extern crate alloc;

    mod buffers;
    mod commands;
    mod descriptors;
    mod devices;
    mod images;
    mod presentation;
pub mod rendering;
    mod setup;

use {
    core::ffi::c_void,
    std::collections::HashMap
};

use {
    anyhow::Result,
    ash::vk,
    winit::event_loop::ActiveEventLoop
};

use {
    buffers::{Buffer, VertexBuffer},
    commands::{CommandBuffers, CommandPool},
    descriptors::{DescriptorPool, DescriptorSet, DescriptorSetLayout, UniformBufferObject},
    devices::{Device, PhysicalDevice},
    presentation::{Surface, Swapchain},
    rendering::{Geometry, Pipeline, RenderPass},
    setup::{Entry, Instance}
};

use {
    dacho_window::Window,
    dacho_mesh_c::MeshComponent,
    dacho_log::{log, create_log, destroy_log}
};

#[cfg(feature = "validation")]
use dacho_vulkan_validation::Debug;


trait VulkanDrop {
    fn drop(&self, device: &Device);
}

pub struct Renderer {
    _entry:                     Entry,
    instance:                   Instance,
    #[cfg(feature = "validation")]
    debug:                      Debug,
    physical_device:            PhysicalDevice,
    device:                     Device,
    surface:                    Surface,
    render_pass:                RenderPass,
    swapchain:                  Swapchain,
    descriptor_set_layout:      DescriptorSetLayout,
    pipelines:                  HashMap<String, Pipeline>,
    ubo:                        Buffer,
    ubo_mapped:            *mut c_void,
    descriptor_pool:            DescriptorPool,
    command_pool:               CommandPool,
    descriptor_set:             DescriptorSet,
    command_buffers:            CommandBuffers
}

impl Renderer {
    pub fn new(
        event_loop:     &ActiveEventLoop,
        window:         &Window,
        mesh_instances: &HashMap<u32, Vec<f32>>
    ) -> Result<Self> {
        create_log!(info);

        let entry                 = Entry               ::new()?;
        let instance              = Instance            ::new(event_loop, &entry)?;
        #[cfg(feature = "validation")]
        let debug                 = Debug               ::new(&entry.raw, &instance.raw)?;
        let physical_device       = PhysicalDevice      ::new(&instance)?;
        let device                = Device              ::new(&instance, &physical_device)?;
        let surface               = Surface             ::new(&entry, &instance, &window.raw)?;
        let render_pass           = RenderPass          ::new(&device)?;
        let swapchain             = Swapchain           ::new(&instance, &device, &surface, &physical_device, &render_pass, window.width, window.height)?;
        let descriptor_set_layout = DescriptorSetLayout ::new(&device)?;
        let command_pool          = CommandPool         ::new(&device)?;
        let (ubo, ubo_mapped)     = UniformBufferObject ::new_mapped_buffer(&instance, &physical_device, &device)?;
        let descriptor_pool       = DescriptorPool      ::new(&device)?;
        let descriptor_set        = DescriptorSet       ::new(&device, &descriptor_pool, &descriptor_set_layout, &ubo)?;
        let command_buffers       = CommandBuffers      ::new(&command_pool, &swapchain, &device)?;

        let pipelines = Self::geometry(
            &instance,
            &physical_device,
            &device,
            &command_pool,
            &descriptor_set_layout,
            &render_pass,
            window.width,
            window.height,
            mesh_instances
        )?;

        log!(debug, "Recording Commands");

        command_buffers.record(
            &device,
            &Pipeline::commands_multiple(&pipelines),
            &render_pass,
            &swapchain,
            &pipelines,
            &descriptor_set
        )?;

        Ok(
            Self {
                _entry: entry,
                instance,
                #[cfg(feature = "validation")]
                debug,
                physical_device,
                device,
                surface,
                render_pass,
                swapchain,
                descriptor_set_layout,
                pipelines,
                ubo,
                ubo_mapped,
                descriptor_pool,
                command_pool,
                descriptor_set,
                command_buffers
            }
        )
    }

    #[expect(clippy::too_many_arguments, reason = "stuff around renderer is bulky rn")]
    fn geometry(
        instance:              &Instance,
        physical_device:       &PhysicalDevice,
        device:                &Device,
        command_pool:          &CommandPool,
        descriptor_set_layout: &DescriptorSetLayout,
        render_pass:           &RenderPass,
        width:                  u16,
        height:                 u16,
        mesh_instances:        &HashMap<u32, Vec<f32>>
    ) -> Result<HashMap::<String, Pipeline>> {
        log!(info, "Preparing Meshes");

        if mesh_instances.is_empty() {
            return Ok(HashMap::new());
        }

        let mut geometries        = HashMap::new();
        let mut shader_info_cache = HashMap::new();

        for mi in mesh_instances {
            let mut data   = MeshComponent::BUILDERS[*mi.0 as usize]();
            data.instances = mi.1.to_vec();

            let geometry = Geometry::new(instance, physical_device, device, command_pool, &mut data, &mut shader_info_cache)?;

            geometries.insert(*mi.0, geometry);
        }

        log!(info, "Creating Pipelines");

        let     shader_info = shader_info_cache.get("default").expect("failed to find the default shader");
        let mut pipeline    = Pipeline::new(device, descriptor_set_layout, width, height, render_pass, shader_info)?;
        pipeline.geometries = geometries;

        let mut pipelines = HashMap::new();
        pipelines.insert(String::from("default"), pipeline);

        Ok(pipelines)
    }

    #[inline]
    pub fn wait_for_device(&self) {
        self.device.wait();
    }

    pub fn update_meshes(&mut self, updated_meshes: &HashMap<u32, Vec<f32>>) -> Result<()> {
        if updated_meshes.is_empty() {
            return Ok(());
        }

        let pipeline = self.pipelines.get_mut("default").expect("failed to get the default pipeline");

        for (mesh_id, instances) in updated_meshes {
            if instances.is_empty() {
                if let Some(geometry) = pipeline.geometries.remove(mesh_id) {
                    geometry.drop(&self.device);
                }

                continue;
            }

            let geometry_option = pipeline.geometries.get_mut(mesh_id);

            if let Some(geometry) = geometry_option {
                geometry.instance_buffer.drop(&self.device);
                geometry.instance_buffer = VertexBuffer::new_buffer(&self.instance, &self.physical_device, &self.device, &self.command_pool, instances)?;
                geometry.instance_count  = u32::try_from(instances.len() / 16)?; // / 16 => temp for the default shader
            } else {
                let mut data   = MeshComponent::BUILDERS[*mesh_id as usize]();
                data.instances = instances.to_vec();

                //                                                                                                               ~~~~ temp ~~~~~~~~~
                let geometry = Geometry::new(&self.instance, &self.physical_device, &self.device, &self.command_pool, &mut data, &mut HashMap::new())?;

                pipeline.geometries.insert(*mesh_id, geometry);
            }
        }

        self.command_buffers.record(
            &self.device,
            &Pipeline::commands_multiple(&self.pipelines),
            &self.render_pass,
            &self.swapchain,
            &self.pipelines,
            &self.descriptor_set
        )?;

        Ok(())
    }

    pub fn redraw(&mut self, time: f32) {
        let (image_index, _) = unsafe {
            self.swapchain.loader
                .acquire_next_image(
                    self.swapchain.raw,
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
            self.device.raw.wait_for_fences(
                &[self.swapchain.may_begin_drawing[self.swapchain.current_image]],
                true,
                u64::MAX
            )
        }
            .expect("Waiting for fences failed");

        unsafe {
            self.device.raw.reset_fences(
                &[self.swapchain.may_begin_drawing[self.swapchain.current_image]]
            )
        }
            .expect("Resetting fences failed");

        let semaphores_available = [self.swapchain.images_available[self.swapchain.current_image]];
        let waiting_stages       = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT              ];
        let command_buffers      = [self.command_buffers.raw[image_index as usize]               ];
        let semaphores_finished  = [self.swapchain.images_finished[self.swapchain.current_image] ];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&semaphores_available)
            .wait_dst_stage_mask(&waiting_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&semaphores_finished);

        unsafe {
            self.device.raw.queue_submit(
                self.device.queue,
                &[*submit_info],
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
        destroy_log!(info);

        self.device.wait();

        self.command_pool.drop(&self.device);

        log!(debug, "Destroying Pipelines");

        for pipeline in self.pipelines.values() {
            pipeline.drop(&self.device);
        }

        self.render_pass .drop(&self.device);
        self.swapchain   .drop(&self.device);

        log!(debug, "Destroying UniformBufferObject");

        self.ubo                   .drop(&self.device);
        self.descriptor_pool       .drop(&self.device);
        self.descriptor_set_layout .drop(&self.device);

        self.device.drop();
        self.surface.drop();

        #[cfg(feature = "validation")]
        self.debug.drop();

        self.instance.drop();
    }
}

