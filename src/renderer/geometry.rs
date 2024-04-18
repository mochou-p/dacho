// dacho/src/renderer/geometry.rs

use anyhow::Result;

use ash::vk;

use super::{
    buffer::{Buffer, IndexBuffer, VertexBuffer},
    command::Command,
    vertex_input::{Type, size_of_types}
};

pub struct GeometryData {
    pub shader:            String,
    pub cull_mode:         vk::CullModeFlags,
        descriptor_set_id: Option<usize>,
    pub vertex_info:       Vec<Type>,
    pub instance_info:     Vec<Type>,
        vertices:          Vec<f32>,
        instances:         Vec<f32>,
        indices:           Vec<u16>
}

impl GeometryData {
    pub fn new(
        shader:            String,
        cull_mode:         vk::CullModeFlags,
        descriptor_set_id: Option<usize>,
        vertex_info:       Vec<Type>,
        instance_info:     Vec<Type>,
        vertices:          Vec<f32>,
        instances:         Vec<f32>,
        indices:           Vec<u16>
    ) -> Self {
        Self {
            shader,
            cull_mode,
            descriptor_set_id,
            vertex_info,
            instance_info,
            vertices,
            instances,
            indices,
        }
    }
}

pub struct Geometry {
    pub shader:            String,
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
        let shader            = data.shader.clone();
        let descriptor_set_id = data.descriptor_set_id;
        let index_count       = data.indices.len() as u32;
        let instance_count    = (data.instances.len() / (size_of_types(&data.instance_info) / 4)) as u32;

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

        Ok(
            Self {
                shader,
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

