// dacho/src/renderer/descriptors/uniform.rs

// crates
use {
    anyhow::Result,
    ash::vk
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
    app::logger::Logger,
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
        ubo_mapped: *mut core::ffi::c_void,
        time:        f32
    ) {
        const FOV:  f32 = 2.0;
        const AR:   f32 = 16.0/9.0;
        const NEAR: f32 = 0.0001;
        const FAR:  f32 = 10000.0;

        let view = glam::Mat4::look_at_rh(glam::Vec3::Z, glam::Vec3::ZERO, glam::Vec3::Y);

        let projection = {
            let x = FOV * AR;

            glam::Mat4::orthographic_rh(-x, x, FOV, -FOV, NEAR, FAR)
        };

        let mut ubo = Self {
            _view:       view,
            _projection: projection,
            _camera_pos: glam::Vec3::NEG_Z.extend(0.0),
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

