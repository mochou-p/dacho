// dacho/core/components/mesh/src/planar/circle.rs

use {
    ash::vk,
    glam::Vec3
};

use crate::GeometryData;


pub fn mesh() -> GeometryData {
    let id     = 1;

    #[expect(clippy::min_ident_chars, reason = "save chars")]
    let p      = Vec3::ZERO;
    let radius = 0.5;
    let points = 50_u16;

    // * 3 -> xyz
    // * 2 -> position, normal
    // + 6 -> one more for the center vertex (1 * 3 * 2)
    let mut vertices = Vec::with_capacity((points * 3 * 2 + 6).into());

    // * 3 -> triangle per point
    let mut indices = Vec::with_capacity((points * 3).into());

    //                           position       normal
    vertices.extend_from_slice(&[p.x, p.y, p.z, 0.0, 0.0, 1.0]);

    let angle_step = 360.0 / f32::from(points);

    for i in 0..points {
        let angle = angle_step.mul_add(f32::from(i), -90.0);

        let x = angle.to_radians().cos().mul_add(radius, p.x);
        let y = angle.to_radians().sin().mul_add(radius, p.y);

        vertices.extend_from_slice(&[x, -y, p.z, 0.0, 0.0, 1.0]);
    }

    let u32points = u32::from(points);

    for i in 1..u32points {
        indices.extend_from_slice(&[0, i, i + 1]);
    }

    indices.extend_from_slice(&[0, u32points, 1]);

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

