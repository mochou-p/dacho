// dacho/src/renderer/images/image.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// crate
use crate::{
    app::logger::Logger,
    renderer::{
        commands::CommandPool,
        devices::{Device, PhysicalDevice},
        setup::Instance,
        VulkanObject
    },
    log
};

pub struct Image {
    raw:    vk::Image,
    memory: vk::DeviceMemory
}

impl Image {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device:          &Device,
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        extent_2d:        vk::Extent2D,
        format:           vk::Format,
        usage:            vk::ImageUsageFlags,
        properties:       vk::MemoryPropertyFlags,
        samples:          vk::SampleCountFlags
    ) -> Result<Self> {
        let extent = vk::Extent3D::builder()
            .width(extent_2d.width)
            .height(extent_2d.height)
            .depth(1)
            .build();

        let create_info = vk::ImageCreateInfo::builder()
            .extent(extent)
            .format(format)
            .usage(usage)
            .image_type(vk::ImageType::TYPE_2D)
            .mip_levels(1)
            .array_layers(1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .samples(samples)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let raw = unsafe { device.raw().create_image(&create_info, None) }?;

        let memory_requirements = unsafe { device.raw().get_image_memory_requirements(raw) };
        let memory_properties   = unsafe { instance.raw().get_physical_device_memory_properties(*physical_device.raw()) };

        let memory_type_index = {
            let mut found  = false;
            let mut result = 0;

            for i in 0..memory_properties.memory_type_count {
                let req = (memory_requirements.memory_type_bits & (1 << i)) != 0;
                let mem = (memory_properties.memory_types[i as usize].property_flags & properties) == properties;

                if req && mem {
                    found  = true;
                    result = i;
                    break;
                }
            }

            if !found {
                log!(panic, "Failed to find a suitable memory type"); panic!();
            }

            result
        };

        let allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.raw().allocate_memory(&allocate_info, None) }?;

        unsafe { device.raw().bind_image_memory(raw, memory, 0) }?;

        Ok(Self { raw, memory })
    }

    #[allow(dead_code)]
    pub fn transition_layout(
        &self,
        device:       &Device,
        command_pool: &CommandPool,
        old_layout:    vk::ImageLayout,
        new_layout:    vk::ImageLayout
    ) -> Result<()> {
        let command_buffer = command_pool.begin_single_time_commands(device)?;

        let (src_am, dst_am, src_stage, dst_stage) = match (old_layout, new_layout) {
            (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => {
                (
                    vk::AccessFlags::empty(),
                    vk::AccessFlags::TRANSFER_WRITE,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::TRANSFER
                )
            },
            (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => {
                (
                    vk::AccessFlags::TRANSFER_WRITE,
                    vk::AccessFlags::SHADER_READ,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::FRAGMENT_SHADER
                )
            },
            _ => { log!(panic, "Invalid layout transition"); panic!(); }
        };

        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(self.raw)
            .subresource_range(subresource_range)
            .src_access_mask(src_am)
            .dst_access_mask(dst_am);

        unsafe {
            device.raw().cmd_pipeline_barrier(
                command_buffer,
                src_stage,
                dst_stage,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[*barrier]
            );
        }

        command_pool.end_single_time_commands(device, command_buffer)?;

        Ok(())
    }
}

impl VulkanObject for Image {
    type RawType = vk::Image;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn device_destroy(&self, device: &Device) {
        unsafe {
            device.raw().destroy_image(self.raw, None);
            device.raw().free_memory(self.memory, None);
        }
    }
}

