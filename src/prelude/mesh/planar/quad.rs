// dacho/src/prelude/mesh/planar/quad.rs

// crates
use {
    anyhow::Result,
    ash::vk
};

// crate
use crate::{
    game::logger::Logger,
    ecs::component::Component,
    prelude::types::{V2, V3},
    renderer::rendering::GeometryData,
    log
};

pub fn mesh(p: V3, size: V2) -> Result<GeometryData> {
    let hs = size * 0.5;

    let vertices: Vec<f32> = vec![
        // position                    normal
        p.x - hs.x, -p.y - hs.y, p.z,  0.0,  0.0,  1.0,
        p.x + hs.x, -p.y - hs.y, p.z,  0.0,  0.0,  1.0,
        p.x + hs.x, -p.y + hs.y, p.z,  0.0,  0.0,  1.0,
        p.x - hs.x, -p.y + hs.y, p.z,  0.0,  0.0,  1.0,
    ];

    let indices: Vec<u32> = vec![2, 1, 0,  0, 3, 2];

    let instances: Vec<f32> = vec![1.0, 1.0, 1.0, 0.5, 0.7];

    let shader       = String::from("default");
    let cull_mode    = vk::CullModeFlags::FRONT;
    let polygon_mode = vk::PolygonMode::FILL;

    let geometry_data = GeometryData::new(
        shader,
        cull_mode,
        polygon_mode,
        vertices,
        instances,
        indices
    );

    Ok(geometry_data)
}

