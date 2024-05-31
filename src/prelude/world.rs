// dacho/src/prelude/world.rs

use anyhow::Result;

use {
    super::{
        shapes::Object,
        dacho_main
    },
    crate::application::scene::{Data, Scene}
};

#[cfg(debug_assertions)]
use crate::{
    application::logger::Logger,
    log
};

pub struct World {
    pub objects: Vec<Object>,
        data:    Data
}

#[allow(clippy::new_without_default)]
impl World {
    pub fn new() -> Self {
        Self { objects: vec![], data: Data::new() }
    }

    pub fn demo() -> Self {
        use super::*;

        let mut world = World::new();

        world.add(&[
                Cube::default()
                    .size(V3::new(5.0, 0.4, 5.0))
                    .anchor(Anchor::Top)
                    .build(),
                Cube::default()
                    .position(V3::X)
                    .size(V3::ONE * 0.2)
                    .color(Color::BLUE)
                    .anchor(Anchor::Bottom)
                    .build(),
                Cube::default()
                    .position(V3::Z)
                    .size(V3::ONE * 0.2)
                    .color(Color::CYAN)
                    .anchor(Anchor::Bottom)
                    .build(),
                Cube::default()
                    .position(V3::XZ.normalize())
                    .size(V3::ONE * 0.2)
                    .color(Color::SKY)
                    .anchor(Anchor::Bottom)
                    .build(),
                Sphere::default()
                    .color(Color::PURPLE)
                    .material(Material::METAL)
                    .anchor(Anchor::Bottom)
                    .build()
        ]);

        world
    }

    pub fn add(&mut self, objects: &[Object]) -> &mut Self {
        self.objects.extend_from_slice(objects);

        self
    }

    #[inline]
    pub fn run(&self) {
        self.run_()
            .expect("failed to run dacho_main");
    }

    #[inline]
    #[tokio::main]
    async fn run_(&self) -> Result<()> {
        match self.data.geometry.len() + self.data.texture.len() {
            0 => { dacho_main(&Scene::build(self).await?).await?; },
            _ => { dacho_main(&self.data).await?; }
        }

        Ok(())
    }

    // TODO: keep primitives as just instructions
    pub fn save(&self, filename: &str) {
        #[cfg(debug_assertions)]
        log!(info, "Saving World `{filename}`");

        std::fs::write(
            format!("assets/.cache/worlds.{}.dacho", filename),
            bincode::serialize(
                &self.save_().expect("failed to build World")
            ).expect("failed to serialize World")
        ).expect("failed to write World to file");

        #[cfg(debug_assertions)]
        log!(info, "Saved")
    }

    #[inline]
    #[tokio::main]
    async fn save_(&self) -> Result<Data> {
        Scene::build(self).await
    }

    pub fn load(filename: &str) -> Self {
        #[cfg(debug_assertions)]
        log!(info, "Loading World `{filename}`");

        let data = bincode::deserialize(
            &std::fs::read(
                format!("assets/.cache/worlds.{filename}.dacho")
            ).expect("failed to read World from file")
        ).expect("failed to deserialize World");

        Self { objects: vec![], data }
    }
}

