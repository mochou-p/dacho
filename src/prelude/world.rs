// dacho/src/prelude/world.rs

// crates
use anyhow::Result;

// super
use super::{
    object::Object,
    dacho_main
};

// crate
use crate::{
    application::{
        logger::Logger,
        scene::{Data, Scene}
    },
    log
};

pub struct World {
    pub objects: Vec<Object>,
        data:    Data
}

#[allow(clippy::new_without_default)]
impl World {
    #[must_use]
    pub fn new() -> Self {
        Self { objects: vec![], data: Data::new() }
    }

    pub fn add(&mut self, objects: &[Object]) -> &mut Self {
        self.objects.extend_from_slice(objects);

        self
    }

    #[allow(clippy::missing_panics_doc)]
    #[inline]
    pub fn run(&self) {
        self.run_()
            .expect("failed to run dacho_main");
    }

    #[inline]
    #[tokio::main]
    async fn run_(&self) -> Result<()> {
        match self.data.geometry.len() {
            0 => { dacho_main(&Scene::build(self).await?).await?; },
            _ => { dacho_main(&self.data).await?; }
        }

        Ok(())
    }

    // TODO: keep primitives as just instructions
    #[allow(clippy::missing_panics_doc)]
    pub fn save(&self, filename: &str) {
        #[cfg(debug_assertions)]
        log!(info, "Saving World `{filename}`");
        
        {
            let mut dir = "target/dacho/";

            if !std::path::Path::new(dir).exists() {
                std::fs::create_dir(dir).unwrap_or_else(|_| panic!("failed to create `{dir}`"));
            }

            dir = "target/dacho/worlds/";

            if !std::path::Path::new(dir).exists() {
                std::fs::create_dir(dir).unwrap_or_else(|_| panic!("failed to create `{dir}`"));
            }
        }

        std::fs::write(
            format!("target/dacho/worlds/{filename}.dacho"),
            bincode::serialize(
                &self.save_().expect("failed to build World")
            ).expect("failed to serialize World")
        ).expect("failed to write World to file");

        #[cfg(debug_assertions)]
        log!(info, "Saved");
    }

    #[inline]
    #[tokio::main]
    async fn save_(&self) -> Result<Data> {
        Scene::build(self).await
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn load(filename: &str) -> Self {
        #[cfg(debug_assertions)]
        log!(info, "Loading World `{filename}`");

        std::fs::read(format!("target/dacho/worlds/{filename}.dacho")).map_or_else(
            |_|    { log!(panic, "World `{filename}` does not exist"); panic!(); },
            |file| {
                let data = bincode::deserialize(&file)
                    .expect("failed to deserialize World");

                Self { objects: vec![], data }
            }
        )
    }
}

