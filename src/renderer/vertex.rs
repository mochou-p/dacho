// dacho/src/renderer/vertex.rs

use ash::vk;

use glam::f32 as glam;

use super::{
    color::{Color, ColorData},
    primitive::CubePosUnit
};

type PositionData = (f32, f32, f32);
pub struct CubePosition(pub CubePosUnit, pub CubePosUnit, pub CubePosUnit);

impl CubePosition {
    const fn as_position_data(&self) -> PositionData {
        (self.0 as f32, self.1 as f32, self.2 as f32)
    }
}

pub struct Vertex {
    _position: glam::Vec3,
    _center:   glam::Vec3,
    _color:    glam::Vec3
}

impl Vertex {
    pub const fn new(
        position: &CubePosition,
        center:   &CubePosition,
        color:    &ColorData
    ) -> Self {
        let pos = position.as_position_data();
        let cen = center.as_position_data();

        let _position = glam::Vec3::new(pos.0, pos.1, pos.2);
        let _center   = glam::Vec3::new(cen.0, cen.1, cen.2);
        let _color    = glam::Vec3::new(color.0, color.1, color.2);

        Self { _position, _center, _color }
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

    fn format_from_size(size: usize) -> vk::Format {
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
        let dummy = Self::new(&CubePosition(0, 0, 0), &CubePosition(0, 0, 0), &Color::BLACK);

        let mut offset = 0;

        let position_size   = std::mem::size_of_val(&dummy._position);
        let position_format = Self::format_from_size(position_size);
        let position_offset = offset as u32;

        offset += position_size;

        let center_size     = std::mem::size_of_val(&dummy._center);
        let center_format   = Self::format_from_size(center_size);
        let center_offset   = offset as u32;

        offset += center_size;

        let color_size      = std::mem::size_of_val(&dummy._color);
        let color_format    = Self::format_from_size(color_size);
        let color_offset    = offset as u32;

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
                .format(center_format)
                .offset(center_offset)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(2)
                .format(color_format)
                .offset(color_offset)
                .build()
        ]
    }
}

