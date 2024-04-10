// dacho/src/renderer/instance.rs

use ash::vk;

use glam::f32 as glam;

use super::{
    vertex::{PositionData, Vertex, format_from_vec_size}
};

pub struct Instance {
    _position: glam::Vec3
}

impl Instance {
    pub const fn new(position: &PositionData) -> Self {
        let _position = glam::Vec3::new(position.0, position.1, position.2);

        Self { _position }
    }

    pub fn binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [
            vk::VertexInputBindingDescription::builder()
                .binding(1)
                .stride(std::mem::size_of::<Instance>() as u32)
                .input_rate(vk::VertexInputRate::INSTANCE)
                .build()
        ]
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 1] {
        let dummy = Self::new(&(0.0, 0.0, 0.0));

        let location = Vertex::attribute_descriptions().len() as u32;

        let position_size   = std::mem::size_of_val(&dummy._position);
        let position_format = format_from_vec_size(position_size);
        let position_offset = 0;

        [
            vk::VertexInputAttributeDescription::builder()
                .binding(1)
                .location(location)
                .format(position_format)
                .offset(position_offset)
                .build()
        ]
    }
}

