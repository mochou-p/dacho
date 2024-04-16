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

pub struct GeometryData {
    pipeline_id:       Option<usize>,
    descriptor_set_id: Option<usize>,
    vertices:          Vec<vi_Vertex>,
    instances:         Vec<vi_Instance>,
    indices:           Vec<u16>
}

impl GeometryData {
    pub fn new(
        pipeline_id:       Option<usize>,
        descriptor_set_id: Option<usize>,
        vertices:          Vec<vi_Vertex>,
        instances:         Vec<vi_Instance>,
        indices:           Vec<u16>
    ) -> Self {
        Self {
            pipeline_id,
            descriptor_set_id,
            vertices,
            instances,
            indices
        }
    }
}


pub struct Geometry {
    pub pipeline_id:       Option<usize>,
    pub descriptor_set_id: Option<usize>,
        vertex_buffer:     Buffer,
        instance_buffer:   Buffer,
        index_buffer:      Buffer,
        index_count:       u32,
        instance_count:    u32
}

impl Geometry {
    pub fn new(
        instance:        &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        device:          &ash::Device,
        queue:           &vk::Queue,
        command_pool:    &vk::CommandPool,
        data:            &GeometryData
    ) -> Result<Self> {
        let pipeline_id       = data.pipeline_id;
        let descriptor_set_id = data.descriptor_set_id;

        let vertex_buffer = VertexBuffer::new(
            instance,
            physical_device,
            device,
            queue,
            command_pool,
            &data.vertices
        )?;

        let instance_buffer = VertexBuffer::new(
            instance,
            physical_device,
            device,
            queue,
            command_pool,
            &data.instances
        )?;

        let index_buffer = IndexBuffer::new(
            instance,
            physical_device,
            device,
            queue,
            command_pool,
            &data.indices
        )?;

        let    index_count = data.indices.len()   as u32;
        let instance_count = data.instances.len() as u32;

        Ok(
            Self {
                pipeline_id,
                descriptor_set_id,
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

