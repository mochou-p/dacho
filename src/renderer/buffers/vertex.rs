// dacho/src/renderer/buffers/vertex.rs

// core
use core::{
    ffi::c_void,
    mem::size_of_val
};

// crates
use {
    anyhow::Result,
    ash::vk
};

// super
use super::{Buffer, StagingBuffer};

// crate
use crate::renderer::{
    commands::CommandPool,
    devices::{Device, PhysicalDevice},
    setup::Instance
};

pub struct VertexBuffer;

impl VertexBuffer {
    pub fn new_buffer(
        instance:        &Instance,
        physical_device: &PhysicalDevice,
        device:          &Device,
        command_pool:    &CommandPool,
        vertices:        &[f32]
    ) -> Result<Buffer> {
        let vertex_buffer = {
            #[allow(clippy::as_ptr_cast_mut)]
            let data        = vertices.as_ptr() as *mut c_void;
            let buffer_size = size_of_val(vertices) as u64;
            let buffer_type = vk::BufferUsageFlags::VERTEX_BUFFER;

            StagingBuffer::new_buffer(
                instance,
                physical_device,
                device,
                command_pool,
                data,
                buffer_size,
                buffer_type
            )?
        };

        Ok(vertex_buffer)
    }
}

