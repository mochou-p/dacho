// dacho/src/prelude/mesh/spatial/cube.rs

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

pub fn mesh(p: V3, size: V3) -> Result<GeometryData> {
    let hs = size * 0.5;

    let vertices: Vec<f32> = vec![
        // position                           normal
        p.x - hs.x, -p.y + hs.y, p.z - hs.z,  0.0,  1.0,  0.0,
        p.x + hs.x, -p.y + hs.y, p.z - hs.z,  0.0,  1.0,  0.0,
        p.x + hs.x, -p.y + hs.y, p.z + hs.z,  0.0,  1.0,  0.0,
        p.x - hs.x, -p.y + hs.y, p.z + hs.z,  0.0,  1.0,  0.0,

        p.x - hs.x, -p.y - hs.y, p.z - hs.z,  0.0, -1.0,  0.0,
        p.x + hs.x, -p.y - hs.y, p.z - hs.z,  0.0, -1.0,  0.0,
        p.x + hs.x, -p.y - hs.y, p.z + hs.z,  0.0, -1.0,  0.0,
        p.x - hs.x, -p.y - hs.y, p.z + hs.z,  0.0, -1.0,  0.0,

        p.x - hs.x, -p.y - hs.y, p.z - hs.z, -1.0,  0.0,  0.0,
        p.x - hs.x, -p.y + hs.y, p.z - hs.z, -1.0,  0.0,  0.0,
        p.x - hs.x, -p.y + hs.y, p.z + hs.z, -1.0,  0.0,  0.0,
        p.x - hs.x, -p.y - hs.y, p.z + hs.z, -1.0,  0.0,  0.0,

        p.x + hs.x, -p.y - hs.y, p.z - hs.z,  1.0,  0.0,  0.0,
        p.x + hs.x, -p.y + hs.y, p.z - hs.z,  1.0,  0.0,  0.0,
        p.x + hs.x, -p.y + hs.y, p.z + hs.z,  1.0,  0.0,  0.0,
        p.x + hs.x, -p.y - hs.y, p.z + hs.z,  1.0,  0.0,  0.0,

        p.x - hs.x, -p.y - hs.y, p.z + hs.z,  0.0,  0.0,  1.0,
        p.x + hs.x, -p.y - hs.y, p.z + hs.z,  0.0,  0.0,  1.0,
        p.x + hs.x, -p.y + hs.y, p.z + hs.z,  0.0,  0.0,  1.0,
        p.x - hs.x, -p.y + hs.y, p.z + hs.z,  0.0,  0.0,  1.0,

        p.x - hs.x, -p.y - hs.y, p.z - hs.z,  0.0,  0.0, -1.0,
        p.x + hs.x, -p.y - hs.y, p.z - hs.z,  0.0,  0.0, -1.0,
        p.x + hs.x, -p.y + hs.y, p.z - hs.z,  0.0,  0.0, -1.0,
        p.x - hs.x, -p.y + hs.y, p.z - hs.z,  0.0,  0.0, -1.0
    ];

    let indices: Vec<u32> = vec![
         0,  1,  2,   2,  3,  0,
         7,  6,  5,   5,  4,  7,
         8,  9, 10,  10, 11,  8,
        15, 14, 13,  13, 12, 15,
        19, 18, 17,  17, 16, 19,
        20, 21, 22,  22, 23, 20
    ];

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

