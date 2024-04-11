// dacho/src/renderer/vertex.rs

use ash::vk;

use glam::f32 as glam;

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

pub fn format_from_vec<T>(_: &T) -> vk::Format {
    static FORMATS: [vk::Format; 4] = [
        vk::Format::R32_SFLOAT,
        vk::Format::R32G32_SFLOAT,
        vk::Format::R32G32B32_SFLOAT,
        vk::Format::R32G32B32A32_SFLOAT
    ];

    let index = std::mem::size_of::<T>() / std::mem::size_of::<f32>() - 1;

    FORMATS[index]
}

