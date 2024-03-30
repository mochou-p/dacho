// dacho/src/renderer/vertex.rs

use ash::vk;

struct Vec2 {
    _x: f32,
    _y: f32
}

struct Vec3 {
    _x: f32,
    _y: f32,
    _z: f32
}

pub struct Vertex {
    _position: Vec2,
    _color:    Vec3
}

impl Vertex {
    pub const fn new(position: (f32, f32), color: (f32, f32, f32)) -> Self {
        Self {
            _position: Vec2 { _x: position.0, _y: position.1 },
            _color:    Vec3 { _x: color.0,    _y: color.1,    _z: color.2 }
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

    pub fn attribute_descriptions() -> [vk::VertexInputAttributeDescription; 2] {
        [
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(std::mem::size_of::<Vec2>() as u32)
                .build()
        ]
    }
}

