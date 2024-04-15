// dacho/src/renderer/geometry.rs

use anyhow::Result;

use ash::vk;

use super::{
    buffer::{Buffer, IndexBuffer, VertexBuffer},
    command::Command,
    vertex_input::{
        vertex::Vertex     as vi_Vertex,
        instance::Instance as vi_Instance
    }
};

pub struct Geometry {
    vertex_buffer:   Buffer,
    instance_buffer: Buffer,
    index_buffer:    Buffer,
    index_count:     u32,
    instance_count:  u32
}

impl Geometry {
    pub fn new(
        instance:        &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        device:          &ash::Device,
        queue:           &vk::Queue,
        command_pool:    &vk::CommandPool,
        vertices:        &Vec<vi_Vertex>,
        instances:       &Vec<vi_Instance>,
        indices:         &Vec<u16>
    ) -> Result<Self> {
        let vertex_buffer = VertexBuffer::new(
            instance,
            physical_device,
            device,
            queue,
            command_pool,
            vertices
        )?;

        let instance_buffer = VertexBuffer::new(
            instance,
            physical_device,
            device,
            queue,
            command_pool,
            instances
        )?;

        let index_buffer = IndexBuffer::new(
            instance,
            physical_device,
            device,
            queue,
            command_pool,
            indices
        )?;

        let    index_count =   indices.len() as u32;
        let instance_count = instances.len() as u32;

        Ok(
            Self {
                vertex_buffer,
                instance_buffer,
                index_buffer,
                index_count,
                instance_count
            }
        )
    }

    pub fn draw(&self) -> Vec<Command> {
        vec![
            Command::BindVertexBuffers(&self.vertex_buffer, &self.instance_buffer),
            Command::BindIndexBuffer(&self.index_buffer),
            Command::DrawIndexed(self.index_count, self.instance_count)
        ]
    }

    pub fn destroy(&self, device: &ash::Device) {
        self.vertex_buffer.destroy(device);
        self.instance_buffer.destroy(device);
        self.index_buffer.destroy(device);
    }
}

