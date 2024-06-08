// dacho/src/application/scene.rs

use {
    anyhow::Result,
    futures::future::join_all,
    serde::{Serialize, Deserialize},
    tokio::spawn
};

use {
    super::logger::Logger,
    crate::{
        prelude::{
            primitives::{cube, sphere},
            shapes::Object::{Cube, Sphere},
            world::World
        },
        renderer::rendering::geometry::*,
        log
    }
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub geometry: Vec<GeometryData>
}

impl Data {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { geometry: vec![] }
    }
}

pub struct Scene;

impl Scene {
    pub async fn build(world: &World) -> Result<Data> {
        #[cfg(debug_assertions)]
        log!(info, "Building Scene");

        let mut futures = vec![];

        for object in world.objects.iter() {
            futures.push(
                match object {
                    Cube   (p, s, c, m) => { spawn(cube   (*p, *s, *c, *m))                },
                    Sphere (p, s, c, m) => { spawn(sphere (*p, *s, *c, *m, 32, 18, "pbr")) }
                }
            );
        }

        let results = join_all(futures).await;

        let mut geometry = vec![];

        for object in results.iter() {
            match object {
                Ok(result) => match result {
                    Ok(result) => { geometry.push(result.clone()); },
                    Err(err)   => { log!(panic, "{err}"); panic!(); }
                },
                Err(err) => { log!(panic, "{err}"); panic!(); }
            }
        }

        Ok(Data { geometry })
    }
}

