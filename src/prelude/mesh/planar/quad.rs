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

pub fn mesh() -> Result<GeometryData> {
    let id = 0;

    let p  = V3::ZERO;
    let hs = V2::ONE * 0.5;

    let vertices: Vec<f32> = vec![
        // position                    normal
        p.x - hs.x, -p.y - hs.y, p.z,  0.0,  0.0,  1.0,
        p.x + hs.x, -p.y - hs.y, p.z,  0.0,  0.0,  1.0,
        p.x + hs.x, -p.y + hs.y, p.z,  0.0,  0.0,  1.0,
        p.x - hs.x, -p.y + hs.y, p.z,  0.0,  0.0,  1.0,
    ];

    let indices: Vec<u32> = vec![2, 1, 0,  0, 3, 2];

    let shader       = String::from("default");
    let cull_mode    = vk::CullModeFlags::FRONT;
    let polygon_mode = vk::PolygonMode::FILL;

    let geometry_data = GeometryData::new(
        shader,
        id,
        cull_mode,
        polygon_mode,
        vertices,
        vec![], // instances
        indices
    );

    Ok(geometry_data)
}

