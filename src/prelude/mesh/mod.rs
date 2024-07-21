// dacho/src/prelude/mesh/mod.rs

// modules
mod planar;
mod spatial;

// crates
use anyhow::Result;

// super
use super::types::{V2, V3};

// crate
use crate::{
    ecs::component::Component,
    renderer::rendering::GeometryData
};

type MeshBuilder = dyn Fn() -> Result<GeometryData>;

pub struct Mesh {
    pub data_builder: Box<MeshBuilder>
}

impl Component for Mesh {}

impl Mesh {
    #[must_use]
    pub fn quad(position: V3, size: V2) -> Self {
        let data_builder = Box::new(
            move || planar::quad::mesh(position, size)
        );

        Self { data_builder }
    }

    #[must_use]
    pub fn circle(position: V3, radius: f32, points: usize, standing: bool) -> Self {
        let data_builder = Box::new(
            move || planar::circle::mesh(position, radius, points, standing)
        );

        Self { data_builder }
    }
}

