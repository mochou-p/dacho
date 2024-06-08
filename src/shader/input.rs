// dacho/src/shader/input.rs

// crates
use {
    anyhow::{Context, Result},
    ash::vk
};

// crate
use crate::{
    application::logger::Logger,
    log
};

#[derive(Clone, Copy, Debug)]
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
    const FORMAT_SIZE_PAIRS: [TypeInfo; 4] = [
        TypeInfo::new(vk::Format::R32_SFLOAT,              std::mem::size_of::<f32>() as u32),
        TypeInfo::new(vk::Format::R32G32_SFLOAT,       2 * std::mem::size_of::<f32>() as u32),
        TypeInfo::new(vk::Format::R32G32B32_SFLOAT,    3 * std::mem::size_of::<f32>() as u32),
        TypeInfo::new(vk::Format::R32G32B32A32_SFLOAT, 4 * std::mem::size_of::<f32>() as u32)
    ];

    const fn new(format: vk::Format, size: u32) -> Self {
        Self { format, size }
    }
}

pub fn wgsl_field_to_type(field: &str) -> Result<Type> {
    let wgsl_type = &field[
        field
            .rfind(' ')
            .context("Failed to parse wgsl field type")?
            +1
        ..
        field.len() - (field.chars().last().context("Failed to get the last char")? == ',') as i32 as usize
    ];

    let kind = match wgsl_type {
        "f32"       => Type::Float,
        "vec2<f32>" => Type::Vec2,
        "vec3<f32>" => Type::Vec3,
        "vec4<f32>" => Type::Vec4,
        _           => { log!(panic, "Unknown glsl type `{wgsl_type}`"); panic!(); }
    };

    Ok(kind)
}

pub fn size_of_types(info: &[Type]) -> usize {
    let mut size = 0;

    for kind in info.iter() {
        size += TypeInfo::FORMAT_SIZE_PAIRS[*kind as usize].size as usize;
    }

    size
}

pub fn vertex_descriptions(info: &[Type]) -> (
    vk::VertexInputBindingDescription, Vec<vk::VertexInputAttributeDescription>, u32
) {
    let  mut attribute_descriptions = Vec::with_capacity(info.len());
    let (mut location, mut offset)  = (0, 0);

    for kind in info.iter() {
        let attribute_description = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(location)
            .format(TypeInfo::FORMAT_SIZE_PAIRS[*kind as usize].format)
            .offset(offset)
            .build();

        location += 1;
        offset   += TypeInfo::FORMAT_SIZE_PAIRS[*kind as usize].size;

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

pub fn instance_descriptions(info: &[Type], location_offset: u32) -> (
    vk::VertexInputBindingDescription, Vec<vk::VertexInputAttributeDescription>
) {
    let  mut attribute_descriptions = Vec::with_capacity(info.len());
    let (mut location, mut offset)  = (location_offset, 0);

    for kind in info.iter() {
        let attribute_description = vk::VertexInputAttributeDescription::builder()
            .binding(1)
            .location(location)
            .format(TypeInfo::FORMAT_SIZE_PAIRS[*kind as usize].format)
            .offset(offset)
            .build();

        location += 1;
        offset   += TypeInfo::FORMAT_SIZE_PAIRS[*kind as usize].size;

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

