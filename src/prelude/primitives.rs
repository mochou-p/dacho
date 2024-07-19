// dacho/src/prelude/primitives.rs

// core
use core::f32::consts::{FRAC_PI_2, PI};

// crates
use {
    anyhow::Result,
    ash::vk
};

// super
use super::types::{V2, V3};

// crate
use crate::{
    game::logger::Logger,
    renderer::rendering::GeometryData,
    log
};

pub async fn quad(p: V3, size: V2) -> Result<GeometryData> {
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

pub async fn circle(p: V3, radius: f32, points: usize) -> Result<GeometryData> {
    if points < 3 {
        log!(panic, "please provide more than 2 Circle.points");
    }

    // * 3 -> xyz
    // * 2 -> position, normal
    // + 6 -> one more for the center vertex (1 * 3 * 2)
    let mut vertices: Vec<f32> = Vec::with_capacity(points * 3 * 2 + 6);

    // * 3 -> triangle per point
    let mut indices:  Vec<u32> = Vec::with_capacity(points * 3);

    //                           position       normal
    vertices.extend_from_slice(&[p.x, p.y, p.z, 0.0, 0.0, 1.0]);

    for i in 0..points {
        let a = (360.0 / points as f32).mul_add(i as f32, -90.0);
        let x = a.to_radians().cos().mul_add(radius, p.x);
        let y = a.to_radians().sin().mul_add(radius, p.y);

        vertices.extend_from_slice(&[x, -y, p.z, 0.0, 0.0, 1.0]);
    }

    let u32points = u32::try_from(points)?;

    for i in 1..u32points {
        indices.extend_from_slice(&[0, i, i + 1]);
    }

    indices.extend_from_slice(&[0, u32points, 1]);

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

pub async fn cube(p: V3, size: V3) -> Result<GeometryData> {
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

pub async fn sphere(position: V3, radius: f32, sectors: usize, stacks: usize) -> Result<GeometryData> {
    // * 3 -> xyz
    // * 2 -> position, normal
    let mut vertices: Vec<f32> = Vec::with_capacity((sectors + 1) * (stacks + 1) * 3 * 2);

    // * 6 -> indices per quad
    let mut indices:  Vec<u32> = Vec::with_capacity(sectors * (stacks - 1) * 6);

    let sector_step = 2.0 * PI / sectors as f32;
    let stack_step  = PI / stacks as f32;

    for i in 0..=stacks {
        let a  = (i as f32).mul_add(-stack_step, FRAC_PI_2);
        let xy = a.cos();
        let z  = a.sin();

        for j in 0..=sectors {
            let a = (j as f32) * sector_step;
            let x = xy * a.cos();
            let y = xy * a.sin();

            vertices.extend_from_slice(
                &[
                    // position
                    x.mul_add(radius,  position.x),
                    y.mul_add(radius, -position.y),
                    z.mul_add(radius,  position.z),

                    // normal
                    x,
                    y,
                    z
                ]
            );
        }
    }

    for i in 0..stacks {
        let mut k1 = u32::try_from(i * (sectors + 1))?;
        let mut k2 = k1 + u32::try_from(sectors)? + 1;

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

