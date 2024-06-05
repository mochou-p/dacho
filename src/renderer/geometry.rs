// dacho/src/renderer/geometry.rs

use {
    std::collections::HashMap,
    anyhow::{Context, Result},
    ash::vk,
    serde::{Serialize, Deserialize}
};

use super::{
    buffer::{Buffer, IndexBuffer, VertexBuffer},
    command::{Command, CommandPool},
    device::{Device, PhysicalDevice},
    instance::Instance,
    pipeline::shader_input_types,
    vertex_input::{ShaderInfo, size_of_types},
    VulkanObject
};

#[derive(Clone, Serialize, Deserialize)]
pub struct GeometryData {
    pub shader:       String,
    pub cull_mode:    u32,
    pub polygon_mode: i32,
    pub vertices:     Vec<f32>,
    pub instances:    Vec<f32>,
        indices:      Vec<u32>
}

impl GeometryData {
    pub const fn new(
        shader:       String,
        cull_mode:    vk::CullModeFlags,
        polygon_mode: vk::PolygonMode,
        vertices:     Vec<f32>,
        instances:    Vec<f32>,
        indices:      Vec<u32>
    ) -> Result<Self> {
        Ok(
            Self {
                shader,
                cull_mode:    cull_mode.as_raw(),
                polygon_mode: polygon_mode.as_raw(),
                vertices,
                instances,
                indices,
            }
        )
    }
}

pub struct Geometry {
    pub shader:          String,
        vertex_buffer:   Buffer,
        instance_buffer: Buffer,
        index_buffer:    Buffer,
        index_count:     u32,
        instance_count:  u32
}

impl Geometry {
    pub fn new(
        instance:          &Instance,
        physical_device:   &PhysicalDevice,
        device:            &Device,
        command_pool:      &CommandPool,
        data:              &GeometryData,
        shader_info_cache: &mut HashMap<String, ShaderInfo>
    ) -> Result<Self> {
        let shader      = data.shader.clone();
        let index_count = data.indices.len() as u32;

        if shader_info_cache.get(&data.shader).is_none() {
            let name         = data.shader.clone();
            let cull_mode    = data.cull_mode;
            let polygon_mode = data.polygon_mode;

            let (vertex_info, instance_info) = shader_input_types(&data.shader)?;
            let instance_size = size_of_types(&instance_info) / std::mem::size_of::<f32>();

            shader_info_cache.insert(
                data.shader.clone(),
                ShaderInfo {
                    name,
                    cull_mode:    vk::CullModeFlags::from_raw(cull_mode),
                    polygon_mode: vk::PolygonMode::from_raw(polygon_mode),
                    vertex_info,
                    instance_info,
                    instance_size
                }
            );
        }

        let instance_count = (
            data.instances.len()
            / shader_info_cache.get(&data.shader)
                .context("Shader instance size cache HashMap error")?
                .instance_size
        ) as u32;

        let vertex_buffer = VertexBuffer::new_buffer(
            instance,
            physical_device,
            device,
            command_pool,
            &data.vertices
        )?;

        let instance_buffer = VertexBuffer::new_buffer(
            instance,
            physical_device,
            device,
            command_pool,
            &data.instances
        )?;

        let index_buffer = IndexBuffer::new_buffer(
            instance,
            physical_device,
            device,
            command_pool,
            &data.indices
        )?;

        Ok(
            Self {
                shader,
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

    pub fn destroy(&self, device: &Device) {
        self.vertex_buffer.destroy(Some(device));
        self.instance_buffer.destroy(Some(device));
        self.index_buffer.destroy(Some(device));
    }
}

