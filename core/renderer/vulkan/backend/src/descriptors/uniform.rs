// dacho/core/renderer/vulkan/backend/src/descriptors/uniform.rs

use core::{ffi::c_void, mem::size_of, ptr::{copy_nonoverlapping, from_mut}};

use {
    anyhow::Result,
    ash::vk,
    glam::f32::{Mat4, Vec4}
};

use crate::{buffers::Buffer, devices::{Device, PhysicalDevice}, setup::Instance};

use {
    dacho_log::create_log,
    dacho_components::Camera
};


pub struct UniformBufferObject {
    _view:       Mat4,
    _projection: Mat4,
    _camera_pos: Vec4,
    _time:       f32
}

impl UniformBufferObject {
    pub fn new_mapped_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device
    ) -> Result<(Buffer, *mut c_void)> {
        create_log!(debug);

        let buffer_size = size_of::<Self>() as u64;

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
        ubo_mapped: *mut c_void,
        time:            f32,
        camera:     &Camera
    ) {
        let mut ubo = Self {
            _view:       camera.view,
            _projection: camera.projection,
            _camera_pos: camera.position.extend(0.0),
            _time:       time
        };

        let src  = from_mut::<Self>(&mut ubo).cast::<c_void>();
        let size = size_of::<Self>();

        unsafe { copy_nonoverlapping(src, ubo_mapped, size); }
    }
}

