// dacho/src/renderer/vertex_input/vertex.rs

use ash::vk;

use glam::f32 as glam;

use super::format_from_vec;

pub struct Vertex {
    _position: glam::Vec4
}

impl Vertex {
    pub const fn new(
        x:       f32,
        y:       f32,
        z:       f32,
        uv_frac: f32
    ) -> Self {
        Self { _position: glam::Vec4::new(x, y, z, uv_frac) }
    }

    pub fn binding_descriptions() -> Vec<vk::VertexInputBindingDescription> {
        vec![
            vk::VertexInputBindingDescription::builder()
                .binding(0)
                .stride(std::mem::size_of::<Self>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
                .build()
        ]
    }

    pub fn attribute_descriptions() -> Vec<vk::VertexInputAttributeDescription> {
        static DUMMY: Vertex = Vertex::new(0.0, 0.0, 0.0, 0.0);

        let position_format = format_from_vec(&DUMMY._position);
        let position_offset = 0;

        vec![
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(position_format)
                .offset(position_offset)
                .build()
        ]
    }
}

