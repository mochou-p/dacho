// dacho/src/renderer/vertex_input/mod.rs

use ash::vk;

#[derive(Clone, Copy)]
pub enum Type {
    Float,
    Vec2,
    Vec3,
    Vec4
}

struct TypeInfo {
    format: vk::Format,
    size:   u32
}

impl TypeInfo {
    const fn new(format: vk::Format, size: u32) -> Self {
        Self { format, size }
    }
}

const TYPE_INFOS: [TypeInfo; 4] = [
    TypeInfo::new(vk::Format::R32_SFLOAT,           4),
    TypeInfo::new(vk::Format::R32G32_SFLOAT,        8),
    TypeInfo::new(vk::Format::R32G32B32_SFLOAT,    12),
    TypeInfo::new(vk::Format::R32G32B32A32_SFLOAT, 16)
];

pub fn size_of_types(info: &Vec<Type>) -> usize {
    let mut size = 0_usize;

    for kind in info.iter() {
        size += TYPE_INFOS[*kind as usize].size as usize;
    }

    size
}

pub fn vertex_descriptions(
    info: &Vec<Type>
) -> (vk::VertexInputBindingDescription, Vec<vk::VertexInputAttributeDescription>, u32) {
    let  mut attribute_descriptions = vec![];
    let (mut location, mut offset)  = (0_u32, 0_u32);

    for kind in info.iter() {
        let attribute_description = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(location)
            .format(TYPE_INFOS[*kind as usize].format)
            .offset(offset)
            .build();

        location += 1;
        offset   += TYPE_INFOS[*kind as usize].size;

        attribute_descriptions.push(attribute_description);
    }

    let stride = offset;

    let binding_description = vk::VertexInputBindingDescription::builder()
        .binding(0)
        .stride(stride)
        .input_rate(vk::VertexInputRate::VERTEX)
        .build();

    (binding_description, attribute_descriptions, location)
}

pub fn instance_descriptions(
    info:                 &Vec<Type>,
    vertex_last_location:  u32
) -> (vk::VertexInputBindingDescription, Vec<vk::VertexInputAttributeDescription>) {
    let  mut attribute_descriptions = vec![];
    let (mut location, mut offset) = (vertex_last_location, 0);

    for kind in info.iter() {
        let attribute_description = vk::VertexInputAttributeDescription::builder()
            .binding(1)
            .location(location)
            .format(TYPE_INFOS[*kind as usize].format)
            .offset(offset)
            .build();

        location += 1;
        offset   += TYPE_INFOS[*kind as usize].size;

        attribute_descriptions.push(attribute_description);
    }

    let stride = offset;

    let binding_description = vk::VertexInputBindingDescription::builder()
        .binding(1)
        .stride(stride)
        .input_rate(vk::VertexInputRate::INSTANCE)
        .build();

    (binding_description, attribute_descriptions)
}

