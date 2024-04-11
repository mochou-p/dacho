// dacho/src/renderer/instance.rs

use ash::vk;

use glam::f32 as glam;

use super::vertex::{Vertex, format_from_vec};

pub struct Instance {
    _position: glam::Vec3
}

impl Instance {
    pub const fn new(
        x: f32,
        y: f32,
        z: f32
    ) -> Self {
        Self { _position: glam::Vec3::new(x, y, z) }
    }

    pub fn binding_descriptions() -> [vk::VertexInputBindingDescription; 1] {
        [
            vk::VertexInputBindingDescription::builder()
                .binding(1)
                .stride(std::mem::size_of::<Self>() as u32)
                .input_rate(vk::VertexInputRate::INSTANCE)
                .build()
        ]
    }

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 1] {
        static DUMMY: Instance = Instance::new(0.0, 0.0, 0.0);

        let location = Vertex::attribute_descriptions().len() as u32;

        let position_format = format_from_vec(&DUMMY._position);
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

