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
        let model      = glam::Mat4::from_axis_angle(glam::Vec3::Z, 90.0_f32.to_radians() * time);
        let view       = glam::Mat4::look_at_lh(glam::Vec3::ONE * 2.0, glam::Vec3::ZERO, glam::Vec3::Z);

        let mut projection   = glam::Mat4::perspective_lh(45.0_f32.to_radians(), aspect_ratio, 0.1, 10.0);
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

