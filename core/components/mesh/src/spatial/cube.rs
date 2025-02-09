// dacho/core/components/mesh/src/spatial/cube.rs

use {
    ash::vk,
    glam::Vec3
};

use crate::GeometryData;


pub fn mesh() -> GeometryData {
    let id = 2;

    #[expect(clippy::min_ident_chars, reason = "save chars")]
    let p  = Vec3::ZERO;
    let hs = Vec3::splat(0.5);

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

