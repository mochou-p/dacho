// dacho/src/prelude/mesh/planar/quad.rs

// crates
use ash::vk;

// crate
use crate::{
    prelude::types::{V2, V3},
    renderer::rendering::GeometryData
};

pub fn mesh() -> GeometryData {
    let id = 0;

    #[allow(clippy::min_ident_chars)]
    let p  = V3::ZERO;
    let hs = V2::ONE * 0.5;

    let vertices = vec![
        // position                    normal
        p.x - hs.x, -p.y - hs.y, p.z,  0.0,  0.0,  1.0,
        p.x + hs.x, -p.y - hs.y, p.z,  0.0,  0.0,  1.0,
        p.x + hs.x, -p.y + hs.y, p.z,  0.0,  0.0,  1.0,
        p.x - hs.x, -p.y + hs.y, p.z,  0.0,  0.0,  1.0,
    ];

    let indices = vec![2, 1, 0,  0, 3, 2];

    let shader       = String::from("default");
    let cull_mode    = vk::CullModeFlags::FRONT;
    let polygon_mode = vk::PolygonMode::FILL;

    GeometryData::new(
        shader,
        id,
        cull_mode,
        polygon_mode,
        vertices,
        vec![], // instances
        indices
    )
}

