// dacho/src/application/scene.rs

// crates
use {
    anyhow::Result,
    futures::future::join_all,
    serde::{Serialize, Deserialize},
    tokio::spawn
};

// super
use super::logger::Logger;

// crate
use crate::{
    prelude::{
        primitives::{cube, sphere},
        shapes::{Camera, Object::{Camera as OCamera, Cube, Sphere}},
        world::World
    },
    renderer::rendering::GeometryData,
    log
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub geometry: Vec<GeometryData>,
    pub camera:   Camera
}

impl Data {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { geometry: vec![], camera: Camera::default() }
    }
}

pub struct Scene;

impl Scene {
    pub async fn build(world: &World) -> Result<Data> {
        #[cfg(debug_assertions)]
        log!(info, "Building Scene");

        let mut futures = vec![];

        let mut camera_option: Option<Camera> = None;

        for object in &world.objects {
            match object {
                Cube    (p, s, c, m) => { futures.push(spawn(cube   (*p, *s, *c, *m        ))) },
                Sphere  (p, s, c, m) => { futures.push(spawn(sphere (*p, *s, *c, *m, 32, 18))) },
                OCamera (p, m)       => { camera_option = Some(Camera::new(*p, m.clone())); }
            }
        }

        let camera = camera_option.unwrap_or_default();

        let results = join_all(futures).await;

        let mut geometry = vec![];

        for object in &results {
            match object {
                Ok(result) => match result {
                    Ok(result) => { geometry.push(result.clone()); },
                    Err(err)   => { log!(panic, "{err}"); panic!(); }
                },
                Err(err) => { log!(panic, "{err}"); panic!(); }
            }
        }

        Ok(Data { geometry, camera })
    }
}

