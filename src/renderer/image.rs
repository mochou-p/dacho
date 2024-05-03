// dacho/src/renderer/image.rs

use {
    anyhow::Result,
    ash::vk
};

use super::{
    buffer::StagingBuffer,
    command::CommandPool,
    device::{Device, PhysicalDevice},
    instance::Instance
};

pub struct Image {
    pub raw:    vk::Image,
        memory: vk::DeviceMemory
}

impl Image {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device:          &Device,
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        extent_2d:       &vk::Extent2D,
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

        let raw = unsafe { device.raw.create_image(&create_info, None) }?;

        let memory_requirements = unsafe { device.raw.get_image_memory_requirements(raw) };
        let memory_properties   = unsafe { instance.raw.get_physical_device_memory_properties(physical_device.raw) };

        let memory_type_index = {
            let mut found  = false;
            let mut result = 0;

            for i in 0..memory_properties.memory_type_count {
                let a = (memory_requirements.memory_type_bits & (1 << i)) != 0;
                let b = (memory_properties.memory_types[i as usize].property_flags & properties) == properties;

                if a && b {
                    found  = true;
                    result = i;
                    break;
                }
            }

            if !found {
                panic!("Failed to find a suitable memory type");
            }

            result
        };

        let allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(memory_type_index);

        let memory = unsafe { device.raw.allocate_memory(&allocate_info, None) }?;

        unsafe { device.raw.bind_image_memory(raw, memory, 0) }?;

        Ok(Self { raw, memory })
    }

    fn transition_layout(
        &self,
        device:       &Device,
        command_pool: &CommandPool,
        old_layout:    vk::ImageLayout,
        new_layout:    vk::ImageLayout
    ) -> Result<()> {
        let command_buffer = command_pool.begin_single_time_commands(device)?;

        let src_stage;
        let dst_stage;
        let src_access_mask;
        let dst_access_mask;

        if old_layout == vk::ImageLayout::UNDEFINED
            && new_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
        {
            src_access_mask = vk::AccessFlags::empty();
            dst_access_mask = vk::AccessFlags::TRANSFER_WRITE;

            src_stage = vk::PipelineStageFlags::TOP_OF_PIPE;
            dst_stage = vk::PipelineStageFlags::TRANSFER;
        } else if old_layout == vk::ImageLayout::TRANSFER_DST_OPTIMAL
            && new_layout == vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
        {
            src_access_mask = vk::AccessFlags::TRANSFER_WRITE;
            dst_access_mask = vk::AccessFlags::SHADER_READ;

            src_stage = vk::PipelineStageFlags::TRANSFER;
            dst_stage = vk::PipelineStageFlags::FRAGMENT_SHADER;
        } else {
            panic!("Invalid layout transition");
        }

        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let barriers = [
            vk::ImageMemoryBarrier::builder()
                .old_layout(old_layout)
                .new_layout(new_layout)
                .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                .image(self.raw)
                .subresource_range(subresource_range)
                .src_access_mask(src_access_mask)
                .dst_access_mask(dst_access_mask)
                .build()
        ];

        unsafe {
            device.raw.cmd_pipeline_barrier(
                command_buffer,
                src_stage,
                dst_stage,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &barriers
            );
        }

        command_pool.end_single_time_commands(device, &command_buffer)?;

        Ok(())
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.raw.destroy_image(self.raw, None);
            device.raw.free_memory(self.memory, None);
        }
    }
}

pub struct ImageView {
    pub raw: vk::ImageView
}

impl ImageView {
    pub fn new(
        device:      &Device,
        image:       &vk::Image,
        format:       vk::Format,
        aspect_mask:  vk::ImageAspectFlags
    ) -> Result<Self> {
        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspect_mask)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .build();

        let create_info = vk::ImageViewCreateInfo::builder()
            .image(*image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .subresource_range(subresource_range);

        let raw = unsafe { device.raw.create_image_view(&create_info, None) }?;

        Ok(Self { raw })
    }

    pub fn destroy(&self, device: &Device) {
        unsafe { device.raw.destroy_image_view(self.raw, None); }
    }
}

pub struct Texture;

impl Texture {
    pub fn new_image(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        image_data:      &[u8],
        is_spherical:     bool
    ) -> Result<Image> {
        let data        = image_data.as_ptr() as *mut std::ffi::c_void;
        let buffer_size = std::mem::size_of_val(image_data) as u64;

        let buffer = StagingBuffer::new_buffer(
            instance,
            physical_device,
            device,
            command_pool,
            data,
            buffer_size,
            vk::BufferUsageFlags::TRANSFER_SRC
        )?;

        let (width, height);

        if is_spherical {
            height = ((buffer_size / 4 / 2) as f32).sqrt() as u32;
            width  = height * 2;
        } else {
            width  = ((buffer_size / 4) as f32).sqrt() as u32;
            height = width;
        }

        let image = Image::new(
            device,
            instance,
            physical_device,
            &vk::Extent2D { width, height },
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::SampleCountFlags::TYPE_1
        )?;

        image.transition_layout(
            device,
            command_pool,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL
        )?;

        buffer.copy_to_image(device, command_pool, &image, width, height)?;

        image.transition_layout(
            device,
            command_pool,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL
        )?;

        buffer.destroy(device);

        Ok(image)
    }
}

pub struct TextureView;

impl TextureView {
    pub fn new_image_view(
        device:  &Device,
        texture: &Image
    ) -> Result<ImageView> {
        let image_view = ImageView::new(
            device,
            &texture.raw,
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageAspectFlags::COLOR
        )?;

        Ok(image_view)
    }
}

pub struct Sampler {
    pub raw: vk::Sampler
}

impl Sampler {
    pub fn new(device: &Device, is_spherical: bool) -> Result<Self> {
        let create_info = vk::SamplerCreateInfo::builder()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(
                if is_spherical {
                    vk::SamplerAddressMode::CLAMP_TO_EDGE
                } else {
                    vk::SamplerAddressMode::REPEAT
                }
            )
            .address_mode_w(vk::SamplerAddressMode::REPEAT)
            .anisotropy_enable(true)
            .max_anisotropy(4.0)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(0.0);

        let raw = unsafe { device.raw.create_sampler(&create_info, None) }?;

        Ok(Self { raw })
    }

    pub fn destroy(&self, device: &Device) {
        unsafe { device.raw.destroy_sampler(self.raw, None); }
    }
}

