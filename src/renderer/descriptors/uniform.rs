// dacho/src/renderer/descriptors/uniform.rs

// crates
use {
    anyhow::Result,
    ash::vk,
    glam::f32 as glam
};

// crate
use crate::renderer::{
    buffers::Buffer,
    devices::{Device, PhysicalDevice},
    setup::Instance,
    VulkanObject
};

// debug
#[cfg(debug_assertions)]
use crate::{
    application::logger::Logger,
    log
};

pub struct UniformBufferObject {
    _view:       glam::Mat4,
    _projection: glam::Mat4,
    _camera_pos: glam::Vec4,
    _time:       f32
}

impl UniformBufferObject {
    pub fn new_mapped_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device
    ) -> Result<(Buffer, *mut core::ffi::c_void)> {
        #[cfg(debug_assertions)]
        log!(info, "Creating UniformBuffer");

        let buffer_size = core::mem::size_of::<Self>() as u64;

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
            device.raw().map_memory(uniform_buffer.memory, 0, buffer_size, vk::MemoryMapFlags::empty())
        }?;

        Ok((uniform_buffer, uniform_buffer_mapped))
    }

    pub fn update(
        ubo_mapped:   *mut core::ffi::c_void,
        position:      glam::Vec3,
        direction:     glam::Vec3,
        time:          f32,
        aspect_ratio:  f32
    ) {
        let view = glam::Mat4::look_at_rh(position, position + direction, glam::Vec3::Y);

        let mut projection   = glam::Mat4::perspective_rh(45.0_f32.to_radians(), aspect_ratio, 0.001, 10000.0);
        projection.y_axis.y *= -1.0;

        let position = glam::Vec4::new(position.x, position.y, position.z, 0.0);

        let mut ubo = Self {
            _view:       view,
            _projection: projection,
            _camera_pos: position,
            _time:       time
        };

        let src  = core::ptr::from_mut::<Self>(&mut ubo).cast::<core::ffi::c_void>();
        let size = core::mem::size_of::<Self>();

        unsafe {
            #[allow(unused_unsafe)] // extra unsafe to compile trough a clippy false positive
            core::ptr::copy_nonoverlapping(src, unsafe { ubo_mapped }, size);
        }
    }
}

