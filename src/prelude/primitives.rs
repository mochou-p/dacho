// dacho/src/prelude/primitives.rs

use {
    anyhow::Result,
    ash::vk
};

use {
    super::types::{V2, V3},
    crate::renderer::geometry::GeometryData
};

pub async fn cube(p: V3, size: V3, color: V3, metrou: V2) -> Result<GeometryData> {
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

    let instances: Vec<f32> = vec![color.x, color.y, color.z, metrou.x, metrou.y];

    let shader       = String::from("pbr");
    let cull_mode    = vk::CullModeFlags::FRONT;
    let polygon_mode = vk::PolygonMode::FILL;

    let geometry_data = GeometryData::new(
        shader,
        cull_mode,
        polygon_mode,
        vertices,
        instances,
        indices
    )?;

    Ok(geometry_data)
}

pub async fn sphere(
    position: V3,
    radius:   f32,
    color:    V3,
    metrou:   V2,
    sectors:  usize,
    stacks:   usize,
    shader:   &str
) -> Result<GeometryData> {
    // * 3 -> xyz
    // * 2 -> position, normal
    let mut vertices: Vec<f32> = Vec::with_capacity((sectors + 1) * (stacks + 1) * 3 * 2);

    // * 6 -> indices per quad
    let mut indices:  Vec<u32> = Vec::with_capacity(sectors * (stacks - 1) * 6);

    let sector_step = 2.0 * std::f32::consts::PI / sectors as f32;
    let stack_step  = std::f32::consts::PI / stacks as f32;

    for i in 0..stacks + 1 {
        let a  = std::f32::consts::FRAC_PI_2 - (i as f32) * stack_step;
        let xy = a.cos();
        let z  = a.sin();

        for j in 0..sectors + 1 {
            let a = (j as f32) * sector_step;
            let x = xy * a.cos();
            let y = xy * a.sin();

            vertices.extend_from_slice(
                &[
                    // position
                    x * radius + position.x,
                    y * radius - position.y,
                    z * radius + position.z,

                    // normal
                    x,
                    y,
                    z
                ]
            );
        }
    }

    for i in 0..stacks {
        let mut k1 = (i * (sectors + 1)) as u32;
        let mut k2 = k1 + sectors as u32 + 1;

        for _j in 0..sectors {
            if i != 0 {
                indices.push(k1 + 1);
                indices.push(k2);
                indices.push(k1);
            }

            if i != stacks - 1 {
                indices.push(k2 + 1);
                indices.push(k2);
                indices.push(k1 + 1);
            }

            k1 += 1;
            k2 += 1;
        }
    }

    let instances: Vec<f32> = vec![color.x, color.y, color.z, metrou.x, metrou.y];

    //                         temp vvvvvv
    let shader       = String::from(shader);
    let cull_mode    = vk::CullModeFlags::FRONT;
    let polygon_mode = vk::PolygonMode::FILL;

    let geometry_data = GeometryData::new(
        shader,
        cull_mode,
        polygon_mode,
        vertices,
        instances,
        indices
    )?;

    Ok(geometry_data)
}

