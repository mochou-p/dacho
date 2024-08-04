// dacho/src/shader/input.rs

// core
use core::mem::size_of;

// crates
use {
    anyhow::{Context, Result},
    ash::vk
};

// crate
use crate::{
    app::logger::Logger,
    log
};

type LastLocation         = u32;
type VertexDescriptions   = (vk::VertexInputBindingDescription, Vec<vk::VertexInputAttributeDescription>, LastLocation);
type InstanceDescriptions = (vk::VertexInputBindingDescription, Vec<vk::VertexInputAttributeDescription>);

#[derive(Clone, Copy, Debug)]
pub enum Type {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat4x4
}

pub struct ShaderInfo {
    pub name:          String,
    pub cull_mode:     vk::CullModeFlags,
    pub polygon_mode:  vk::PolygonMode,
    pub vertex_info:   Vec<Type>,
    pub instance_info: Vec<Type>,
    pub instance_size: usize
}

const fn type_to_size(kind: Type) -> usize {
    match kind {
        Type::Float  =>      size_of::<f32>(),
        Type::Vec2   =>  2 * size_of::<f32>(),
        Type::Vec3   =>  3 * size_of::<f32>(),
        Type::Vec4   =>  4 * size_of::<f32>(),
        Type::Mat4x4 => 16 * size_of::<f32>()
    }
}

const fn type_to_format(kind: Type) -> vk::Format {
    match kind {
        Type::Float  => vk::Format::R32_SFLOAT,
        Type::Vec2   => vk::Format::R32G32_SFLOAT,
        Type::Vec3   => vk::Format::R32G32B32_SFLOAT,
        Type::Vec4   => vk::Format::R32G32B32A32_SFLOAT,
        Type::Mat4x4 => panic!("Mat4x4 is not a supported vk::Format")
    }
}

pub fn wgsl_field_to_type(field: &str) -> Result<Type> {
    #[allow(clippy::cast_sign_loss)]
    let wgsl_type = &field[
        field
            .rfind(' ')
            .context("Failed to parse wgsl field type")?
            +1
        ..
        field.len() - i32::from(field.chars().last().context("Failed to get the last char")? == ',') as usize
    ];

    let kind = match wgsl_type {
        "f32"         => Type::Float,
        "vec2<f32>"   => Type::Vec2,
        "vec3<f32>"   => Type::Vec3,
        "vec4<f32>"   => Type::Vec4,
        "mat4x4<f32>" => Type::Mat4x4,
        _             => { log!(panic, "Unknown glsl type `{wgsl_type}`"); panic!(); }
    };

    Ok(kind)
}

pub fn size_of_types(info: &[Type]) -> usize {
    let mut size = 0;

    for kind in info {
        size += type_to_size(*kind);
    }

    size
}

pub fn vertex_descriptions(info: &[Type]) -> Result<VertexDescriptions> {
    let  mut attribute_descriptions = Vec::with_capacity(info.len());
    let (mut location, mut offset)  = (0, 0);

    for kind in info {
        let attribute_description = vk::VertexInputAttributeDescription::builder()
            .binding(0)
            .location(location)
            .format(type_to_format(*kind))
            .offset(offset)
            .build();

        location += 1;
        offset   += u32::try_from(type_to_size(*kind))?;

        attribute_descriptions.push(attribute_description);
    }

    let stride = offset;

    let binding_description = vk::VertexInputBindingDescription::builder()
        .binding(0)
        .stride(stride)
        .input_rate(vk::VertexInputRate::VERTEX)
        .build();

    Ok((binding_description, attribute_descriptions, location))
}

pub fn instance_descriptions(info: &[Type], location_offset: LastLocation) -> Result<InstanceDescriptions> {
    let  mut attribute_descriptions = Vec::with_capacity(info.len());
    let (mut location, mut offset)  = (location_offset, 0);

    for kind in info {
        if matches!(kind, Type::Mat4x4) {
            let row_kind = Type::Vec4;

            for _ in 0..4 {
                let attribute_description = vk::VertexInputAttributeDescription::builder()
                    .binding(1)
                    .location(location)
                    .format(type_to_format(row_kind))
                    .offset(offset)
                    .build();

                location += 1;
                offset   += u32::try_from(type_to_size(row_kind))?;

                attribute_descriptions.push(attribute_description);
            }
        } else {
            let attribute_description = vk::VertexInputAttributeDescription::builder()
                .binding(1)
                .location(location)
                .format(type_to_format(*kind))
                .offset(offset)
                .build();

            location += 1;
            offset   += u32::try_from(type_to_size(*kind))?;

            attribute_descriptions.push(attribute_description);
        }
    }

    let stride = offset;

    let binding_description = vk::VertexInputBindingDescription::builder()
        .binding(1)
        .stride(stride)
        .input_rate(vk::VertexInputRate::INSTANCE)
        .build();

    Ok((binding_description, attribute_descriptions))
}

