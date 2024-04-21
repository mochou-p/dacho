// dacho/src/renderer/vertex_input/mod.rs

use ash::vk;

#[derive(Clone, Copy)]
pub enum Type {
    Float,
    Vec2,
    Vec3,
    Vec4
}

pub struct ShaderInfo {
    pub name:          String,
    pub cull_mode:     vk::CullModeFlags,
    pub polygon_mode:  vk::PolygonMode,
    pub vertex_info:   Vec<Type>,
    pub instance_info: Vec<Type>,
    pub instance_size: usize
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
    TypeInfo::new(vk::Format::R32_SFLOAT,              std::mem::size_of::<f32>() as u32),
    TypeInfo::new(vk::Format::R32G32_SFLOAT,       2 * std::mem::size_of::<f32>() as u32),
    TypeInfo::new(vk::Format::R32G32B32_SFLOAT,    3 * std::mem::size_of::<f32>() as u32),
    TypeInfo::new(vk::Format::R32G32B32A32_SFLOAT, 4 * std::mem::size_of::<f32>() as u32)
];

pub fn str_to_type(string: &str) -> Type {
    match string {
        "float" => Type::Float,
        "vec2"  => Type::Vec2,
        "vec3"  => Type::Vec3,
        "vec4"  => Type::Vec4,
        _       => { panic!("Unknown glsl type '{string}'"); }
    }
}

pub fn size_of_types(info: &[Type]) -> usize {
    let mut size = 0_usize;

    for kind in info.iter() {
        size += TYPE_INFOS[*kind as usize].size as usize;
    }

    size
}

pub fn vertex_descriptions(
    info: &[Type]
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
    info:                 &[Type],
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

