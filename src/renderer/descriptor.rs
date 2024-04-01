// dacho/src/renderer/descriptor.rs

use anyhow::Result;

use ash::vk;

use glam::f32 as glam;

use super::buffer::Buffer;

pub struct UniformBufferObject {
    _model:      glam::Mat4,
    _view:       glam::Mat4,
    _projection: glam::Mat4
}

impl UniformBufferObject {
    pub fn new(
        instance:        &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        device:          &ash::Device
    ) -> Result<(vk::Buffer, vk::DeviceMemory, *mut std::ffi::c_void)> {
        let buffer_size = std::mem::size_of::<UniformBufferObject>() as u64;

        let (uniform_buffer, uniform_buffer_memory) = {
            let usage       = vk::BufferUsageFlags::UNIFORM_BUFFER;
            let properties  = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;

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
            device.map_memory(uniform_buffer_memory, 0, buffer_size, vk::MemoryMapFlags::empty())
        }?;

        Ok((uniform_buffer, uniform_buffer_memory, uniform_buffer_mapped))
    }

    pub fn update(
        ubo_mapped:   *mut std::ffi::c_void,
        time:         f32,
        aspect_ratio: f32
    ) {
        let model = glam::Mat4::from_axis_angle(glam::Vec3::Y, 90.0_f32.to_radians() * time);
        let view  = glam::Mat4::look_at_rh(glam::Vec3::ONE * 2.0, glam::Vec3::ZERO, glam::Vec3::Y);

        let mut projection   = glam::Mat4::perspective_rh(45.0_f32.to_radians(), aspect_ratio, 0.1, 10.0);
        projection.y_axis.y *= -1.0;

        let mut ubo = UniformBufferObject { _model: model, _view: view, _projection: projection };

        let src = &mut ubo
            as *mut UniformBufferObject
            as *mut std::ffi::c_void;

        let size = std::mem::size_of::<UniformBufferObject>();

        unsafe { std::ptr::copy_nonoverlapping(src, ubo_mapped, size); }
    }
}

pub struct DescriptorSetLayout {
    pub descriptor_set_layout: vk::DescriptorSetLayout
}

impl DescriptorSetLayout {
    pub fn new(device: &ash::Device) -> Result<Self> {
        let descriptor_set_layout = {
            let ubo_bindings = [
                vk::DescriptorSetLayoutBinding::builder()
                    .binding(0)
                    .descriptor_count(1)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                    .stage_flags(vk::ShaderStageFlags::VERTEX)
                    .build()
            ];

            let create_info = vk::DescriptorSetLayoutCreateInfo::builder()
                .bindings(&ubo_bindings);

            unsafe { device.create_descriptor_set_layout(&create_info, None) }?
        };

        Ok(Self { descriptor_set_layout })
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe { device.destroy_descriptor_set_layout(self.descriptor_set_layout, None); }
    }
}

pub struct DescriptorPool {
    descriptor_pool: vk::DescriptorPool
}

impl DescriptorPool {
    pub fn new(device: &ash::Device) -> Result<Self> {
        let descriptor_pool = {
            let pool_sizes = [
                vk::DescriptorPoolSize::builder()
                    .ty(vk::DescriptorType::UNIFORM_BUFFER)
                    .descriptor_count(1)
                    .build()
            ];

            let create_info = vk::DescriptorPoolCreateInfo::builder()
                .pool_sizes(&pool_sizes)
                .max_sets(1);

            unsafe { device.create_descriptor_pool(&create_info, None) }?
        };

        Ok(Self { descriptor_pool })
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe { device.destroy_descriptor_pool(self.descriptor_pool, None); }
    }
}

pub struct DescriptorSet;

impl DescriptorSet {
    pub fn new(
        device:                &ash::Device,
        descriptor_pool:       &DescriptorPool,
        descriptor_set_layout: &DescriptorSetLayout,
        ubo:                   &vk::Buffer
    ) -> Result<vk::DescriptorSet> {
        let descriptor_set = {
            let set_layouts = [descriptor_set_layout.descriptor_set_layout];

            let allocate_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(descriptor_pool.descriptor_pool)
                .set_layouts(&set_layouts);

            unsafe { device.allocate_descriptor_sets(&allocate_info) }?[0]
        };

        let buffer_infos = [
            vk::DescriptorBufferInfo::builder()
                .buffer(*ubo)
                .offset(0)
                .range(std::mem::size_of::<UniformBufferObject>() as u64)
                .build()
        ];

        let writes = [
            vk::WriteDescriptorSet::builder()
                .dst_set(descriptor_set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&buffer_infos)
                .build()
        ];

        unsafe { device.update_descriptor_sets(&writes, &[]); }

        Ok(descriptor_set)
    }
}

