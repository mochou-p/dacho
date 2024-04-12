// dacho/src/renderer/vertex_input/mod.rs

pub mod instance;
pub mod vertex;

use ash::vk;

fn format_from_vec<T>(_: &T) -> vk::Format {
    static FORMATS: [vk::Format; 4] = [
        vk::Format::R32_SFLOAT,
        vk::Format::R32G32_SFLOAT,
        vk::Format::R32G32B32_SFLOAT,
        vk::Format::R32G32B32A32_SFLOAT
    ];

    let index = std::mem::size_of::<T>() / std::mem::size_of::<f32>() - 1;

    FORMATS[index]
}

