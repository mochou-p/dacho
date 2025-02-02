// dacho/core/renderer/vulkan/backend/src/rendering/geometry.rs

use {
    core::mem::size_of,
    std::collections::HashMap
};

use {
    anyhow::{Context as _, Result},
    ash::vk
};

use {
    super::shader_input_types,
    crate::{buffers::{Buffer, IndexBuffer, VertexBuffer}, commands::{Command, CommandPool}, devices::{Device, PhysicalDevice}, setup::Instance, VulkanDrop}
};

use {
    dacho_components::GeometryData,
    dacho_shader::{ShaderInfo, size_of_types}
};


#[derive(Hash)]
#[non_exhaustive]
pub struct Geometry {
    pub shader:          String,
    pub id:              u32,
    pub vertex_buffer:   Buffer,
    pub instance_buffer: Buffer,
    pub index_buffer:    Buffer,
    pub index_count:     u32,
    pub instance_count:  u32
}

impl Geometry {
    pub fn new(
        instance:          &Instance,
        physical_device:   &PhysicalDevice,
        device:            &Device,
        command_pool:      &CommandPool,
        data:              &mut GeometryData,
        shader_info_cache: &mut HashMap<String, ShaderInfo>
    ) -> Result<Self> {
        let shader      = data.shader.clone();
        let id          = data.id;
        let index_count = u32::try_from(data.indices.len())?;

        if shader_info_cache.get(&data.shader).is_none() {
            let name                         = data.shader.clone();
            let cull_mode                    = vk::CullModeFlags::from_raw(data.cull_mode);
            let polygon_mode                 = vk::PolygonMode::from_raw(data.polygon_mode);
            let (vertex_info, instance_info) = shader_input_types(&data.shader)?;
            let instance_size                = size_of_types(&instance_info) / size_of::<f32>();

            shader_info_cache.insert(
                data.shader.clone(),
                ShaderInfo {
                    name,
                    cull_mode,
                    polygon_mode,
                    vertex_info,
                    instance_info,
                    instance_size
                }
            );
        }

        let instance_count = u32::try_from(
            data.instances.len()
            / shader_info_cache.get(&data.shader)
                .context("Shader instance size cache HashMap error")?
                .instance_size
        )?;

        let   vertex_buffer = VertexBuffer::new_buffer(instance, physical_device, device, command_pool, &data.vertices )?;
        let instance_buffer = VertexBuffer::new_buffer(instance, physical_device, device, command_pool, &data.instances)?;
        let    index_buffer =  IndexBuffer::new_buffer(instance, physical_device, device, command_pool, &data.indices  )?;

        Ok(Self { shader, id, vertex_buffer, instance_buffer, index_buffer, index_count, instance_count })
    }

    pub fn draw(&self) -> Vec<Command> {
        vec![
            Command::BindVertexBuffers(self.vertex_buffer.raw, self.instance_buffer.raw),
            Command::BindIndexBuffer(self.index_buffer.raw),
            Command::DrawIndexed(self.index_count, self.instance_count)
        ]
    }
}

impl VulkanDrop for Geometry {
    fn drop(&self, device: &Device) {
        self.   vertex_buffer.drop(device);
        self. instance_buffer.drop(device);
        self.    index_buffer.drop(device);
    }
}

