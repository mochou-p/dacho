// dacho/src/renderer/descriptor.rs

use {
    anyhow::Result,
    ash::vk,
    glam::f32 as glam
};

use super::{
    buffer::Buffer,
    device::{Device, PhysicalDevice},
    image::{ImageView, Sampler},
    instance::Instance
};

#[cfg(debug_assertions)]
use crate::{
    application::logger::Logger,
    log
};

pub struct UniformBufferObject {
    _view:       glam::Mat4,
    _projection: glam::Mat4,
    _camera_pos: glam::Vec4,
    _light_pos:  glam::Vec4,
    _time:       f32
}

impl UniformBufferObject {
    pub fn new_mapped_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device
    ) -> Result<(Buffer, *mut std::ffi::c_void)> {
        #[cfg(debug_assertions)]
        log!(info, "Creating UniformBuffer");

        let buffer_size = std::mem::size_of::<UniformBufferObject>() as u64;

        let uniform_buffer = {
            let usage      = vk::BufferUsageFlags::UNIFORM_BUFFER;
            let properties = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;

            Buffer::new(
                instance,
                physical_device,
                device,
                buffer_size,
                usage,
                properties
            )?
        };

        let uniform_buffer_mapped = unsafe {
            device.raw.map_memory(uniform_buffer.memory, 0, buffer_size, vk::MemoryMapFlags::empty())
        }?;

        Ok((uniform_buffer, uniform_buffer_mapped))
    }

    pub fn update(
        ubo_mapped:   *mut std::ffi::c_void,
        position:      glam::Vec3,
        direction:     glam::Vec3,
        time:          f32,
        aspect_ratio:  f32
    ) {
        let view = glam::Mat4::look_at_rh(position, position + direction, glam::Vec3::Y);

        let mut projection   = glam::Mat4::perspective_rh(45.0_f32.to_radians(), aspect_ratio, 0.1, 10000.0);
        projection.y_axis.y *= -1.0;

        let steps      = 10;
        let light_dist = steps as f32 / 2.0 + steps as f32 * 0.1;
        let position   = glam::Vec4::new(position.x, position.y, position.z, 0.0);
        let light_pos  = glam::Vec4::new(time.sin() * light_dist, 0.0, time.cos() * light_dist, 0.0);

        let mut ubo = UniformBufferObject {
            _view:       view,
            _projection: projection,
            _camera_pos: position,
            _light_pos:  light_pos,
            _time:       time
        };

        let src  = &mut ubo as *mut UniformBufferObject as *mut std::ffi::c_void;
        let size = std::mem::size_of::<UniformBufferObject>();

        unsafe { std::ptr::copy_nonoverlapping(src, ubo_mapped, size); }
    }
}

pub struct DescriptorSetLayout {
    pub raw: vk::DescriptorSetLayout
}

impl DescriptorSetLayout {
    pub fn new(device: &Device) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating DescriptorSetLayout");

        let raw = {
            let ubo_bindings = [
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(0)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .stage_flags(vk::ShaderStageFlags::VERTEX)
                    .build(),
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(1)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::SAMPLER)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                    .build(),
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(2)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                    .stage_flags(vk::ShaderStageFlags::FRAGMENT)
                    .build()
            ];

            let create_info = vk::DescriptorSetLayoutCreateInfo::builder()
                .bindings(&ubo_bindings);

            unsafe { device.raw.create_descriptor_set_layout(&create_info, None) }?
        };

        Ok(Self { raw })
    }

    pub fn destroy(&self, device: &Device) {
        #[cfg(debug_assertions)]
        log!(info, "Destroying DescriptorSetLayout");

        unsafe { device.raw.destroy_descriptor_set_layout(self.raw, None); }
    }
}

pub struct DescriptorPool {
    raw: vk::DescriptorPool
}

impl DescriptorPool {
    pub fn new(device: &Device) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating DescriptorPool");

        let raw = {
            let pool_sizes = [
                vk::DescriptorPoolSize::builder()
                    .ty(vk::DescriptorType::UNIFORM_BUFFER)
                    .descriptor_count(1)
                    .build(),
                vk::DescriptorPoolSize::builder()
                    .ty(vk::DescriptorType::SAMPLER)
                    .descriptor_count(1)
                    .build(),
                vk::DescriptorPoolSize::builder()
                    .ty(vk::DescriptorType::SAMPLED_IMAGE)
                    .descriptor_count(1)
                    .build()
            ];

            let create_info = vk::DescriptorPoolCreateInfo::builder()
                .pool_sizes(&pool_sizes)
                .max_sets(1);

            unsafe { device.raw.create_descriptor_pool(&create_info, None) }?
        };

        Ok(Self { raw })
    }

    pub fn destroy(&self, device: &Device) {
        #[cfg(debug_assertions)]
        log!(info, "Destroying DescriptorPool");

        unsafe { device.raw.destroy_descriptor_pool(self.raw, None); }
    }
}

pub struct DescriptorSet {
    pub raw: vk::DescriptorSet
}

impl DescriptorSet {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        device:                &Device,
        descriptor_pool:       &DescriptorPool,
        descriptor_set_layout: &DescriptorSetLayout,
        ubo:                   &Buffer,
        sampler:               &Sampler,
        image_view:            &ImageView
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating DescriptorSet");

        let raw = {
            let set_layouts = [descriptor_set_layout.raw];

            let allocate_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(descriptor_pool.raw)
                .set_layouts(&set_layouts);

            unsafe { device.raw.allocate_descriptor_sets(&allocate_info) }?[0]
        };

        let buffer_infos = [
            vk::DescriptorBufferInfo::builder()
                .buffer(ubo.raw)
                .offset(0)
                .range(std::mem::size_of::<UniformBufferObject>() as u64)
                .build()
        ];

        let sampler_infos = [
            vk::DescriptorImageInfo::builder()
                .image_view(vk::ImageView::null())
                .sampler(sampler.raw)
                .build()
        ];

        let image_view_infos = [
            vk::DescriptorImageInfo::builder()
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image_view(image_view.raw)
                .sampler(vk::Sampler::null())
                .build()
        ];

        let writes = [
            vk::WriteDescriptorSet::builder()
                .dst_set(raw)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&buffer_infos)
                .build(),
            vk::WriteDescriptorSet::builder()
                .dst_set(raw)
                .dst_binding(1)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::SAMPLER)
                .image_info(&sampler_infos)
                .build(),
            vk::WriteDescriptorSet::builder()
                .dst_set(raw)
                .dst_binding(2)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&image_view_infos)
                .build()
        ];

        unsafe { device.raw.update_descriptor_sets(&writes, &[]); }

        Ok(Self { raw })
    }
}

