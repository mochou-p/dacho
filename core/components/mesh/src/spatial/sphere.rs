// dacho/core/components/mesh/src/spatial/sphere.rs

use core::f32::consts::{FRAC_PI_2, PI};

use {
    ash::vk,
    glam::Vec3
};

use crate::GeometryData;


pub fn mesh() -> GeometryData {
    let id = 3;

    let sectors = 8;
    let stacks  = 8;

    let radius   = 1.0;
    let position = Vec3::ZERO;

    // * 3 -> xyz
    // * 2 -> position, normal
    let mut vertices: Vec<f32> = Vec::with_capacity((sectors + 1) * (stacks + 1) * 3 * 2);

    // * 6 -> indices per quad
    let mut indices:  Vec<u32> = Vec::with_capacity(sectors * (stacks - 1) * 6);

    let sector_step = 2.0 * PI / sectors as f32;
    let stack_step  = PI / stacks as f32;

    for i in 0..=stacks {
        let mut angle = (i as f32).mul_add(-stack_step, FRAC_PI_2);
        let     xy    = angle.cos();
        let     z     = angle.sin();

        for j in 0..=sectors {
            angle = (j as f32) * sector_step;
            let x = xy * angle.cos();
            let y = xy * angle.sin();

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

    #[expect(clippy::unwrap_used, reason = "temp")]
    for i in 0..stacks {
        let mut k1 = u32::try_from(i * (sectors + 1)).unwrap();
        let mut k2 = k1 + u32::try_from(sectors).unwrap() + 1;

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

