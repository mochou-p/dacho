// dacho/core/components/mesh/src/planar/quad.rs

use ash::vk;

use crate::GeometryData;

use glam::{Vec2, Vec3};


pub fn mesh() -> GeometryData {
    let id = 0;

    #[expect(clippy::min_ident_chars, reason = "save chars")]
    let p  = Vec3::ZERO;
    let hs = Vec2::ONE * 0.5;

    let vertices = vec![
        // position                   normal
        p.x - hs.x, -p.y - hs.y, p.z, 0.0, 0.0, 1.0,
        p.x + hs.x, -p.y - hs.y, p.z, 0.0, 0.0, 1.0,
        p.x + hs.x, -p.y + hs.y, p.z, 0.0, 0.0, 1.0,
        p.x - hs.x, -p.y + hs.y, p.z, 0.0, 0.0, 1.0,
    ];

    let indices = vec![
        2, 1, 0,
        0, 3, 2
    ];

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

