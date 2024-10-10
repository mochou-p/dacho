// dacho/core/components/mesh/src/spatial/sphere.rs

use core::f32::consts::{FRAC_PI_2, PI};

use {
    anyhow::Result,
    ash::vk
};

use crate::{
    game::logger::Logger,
    ecs::component::Component,
    prelude::types::{V2, V3},
    renderer::rendering::GeometryData,
    log
};


pub fn mesh(position: V3, radius: f32, sectors: usize, stacks: usize) -> Result<GeometryData> {
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

        for _ in 0..sectors {
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

