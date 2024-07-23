// dacho/src/prelude/mesh/planar/circle.rs

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

pub fn mesh(p: V3, radius: f32, points: usize, standing: bool) -> Result<GeometryData> {
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

    let angle_step      = 360.0 / points as f32;
    let one_over_points =   1.0 / points as f32;

    for i in 0..points {
        let a = angle_step.mul_add(
            i as f32,
            if standing {
                180.0_f32.mul_add(one_over_points, -90.0)
            } else {
                -90.0
            }
        );

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
