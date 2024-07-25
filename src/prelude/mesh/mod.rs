// dacho/src/prelude/mesh/mod.rs

// modules
mod planar;
mod spatial;

// crates
use {
    anyhow::Result,
    glam::f32::{Mat4, Quat, Vec3},
};

// super
use super::types::{V2, V3};

// crate
use crate::{
    ecs::{component::Component, world::Id},
    renderer::rendering::GeometryData
};

type MeshBuilder = dyn Fn() -> Result<GeometryData>;

pub struct Mesh {
    pub mesh_id:      Id, // for instancing
    pub model_matrix: Mat4
}

impl Component for Mesh {}

impl Mesh {
    pub const BUILDERS: [&'static MeshBuilder; 2] = [
        &planar::quad::mesh,
        &planar::circle::mesh
    ];

    #[must_use]
    pub fn quad(position: V3, size: V2) -> Self {
        let mesh_id = 0;

        let model_matrix = Mat4::from_scale_rotation_translation(
            size.to_glam().extend(1.0),
            Quat::IDENTITY,
            position.to_glam()
        );

        Self { mesh_id, model_matrix }
    }

    #[must_use]
    pub fn circle(position: V3, radius: f32, points: usize, standing: bool) -> Self {
        let mesh_id = 1;

        let model_matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(radius, radius, 1.0),
            Quat::IDENTITY,
            position.to_glam()
        );

        Self { mesh_id, model_matrix }
    }
}

