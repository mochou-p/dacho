// dacho/src/renderer/vertex.rs

use ash::vk;

use glam::f32 as glam;

use super::color::{Color, ColorData};

type PositionData = (f32, f32, f32);

#[derive(Clone, Copy)]
pub struct Vertex {
    _position:     glam::Vec3,
    _color:        glam::Vec3,
    _normal_index: u32
}

impl Vertex {
    pub const fn new(
        position:     &PositionData,
        color:        &ColorData,
        normal_index:  u32
    ) -> Self {
        let _position = glam::Vec3::new(position.0, position.1, position.2);
        let _color    = glam::Vec3::new(   color.0,    color.1,    color.2);

        Self {
            _position,
            _color,
            _normal_index: normal_index
        }
    }

    pub fn binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [
            vk::VertexInputBindingDescription::builder()
                .binding(0)
                .stride(std::mem::size_of::<Vertex>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX)
                .build()
        ]
    }

    fn format_from_vec_size(size: usize) -> vk::Format {
        static FORMATS: [vk::Format; 4] = [
            vk::Format::R32_SFLOAT,
            vk::Format::R32G32_SFLOAT,
            vk::Format::R32G32B32_SFLOAT,
            vk::Format::R32G32B32A32_SFLOAT
        ];

        let index = (size / std::mem::size_of::<f32>()) - 1;

        FORMATS[index]
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 3] {
        let dummy = Self::new(&(0.0, 0.0, 0.0), &Color::BLACK, 0);

        let mut offset = 0;

        let position_size       = std::mem::size_of_val(&dummy._position);
        let position_format     = Self::format_from_vec_size(position_size);
        let position_offset     = offset as u32;

        offset += position_size;

        let color_size          = std::mem::size_of_val(&dummy._color);
        let color_format        = Self::format_from_vec_size(color_size);
        let color_offset        = offset as u32;

        offset += color_size;

        let normal_index_format = vk::Format::R32_UINT;
        let normal_index_offset = offset as u32;

        [
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(position_format)
                .offset(position_offset)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(color_format)
                .offset(color_offset)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(2)
                .format(normal_index_format)
                .offset(normal_index_offset)
                .build()
        ]
    }
}

