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
        objects::{Camera, Object, Shape::{Cube, Sphere}},
        world::World
    },
    renderer::rendering::GeometryData,
    log
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub geometry: Vec<GeometryData>,
    pub camera:   Object
}

impl Data {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self { geometry: vec![], camera: Camera::DEFAULT_3D }
    }
}

pub struct Scene;

impl Scene {
    pub async fn build(world: &World) -> Result<Data> {
        #[cfg(debug_assertions)]
        log!(info, "Building Scene");

        let mut futures = vec![];

        let mut camera_option: Option<Object> = None;

        for object in &world.objects {
            match object {
                Object::Shape(shape) => match shape {
                    Cube   { position, size   } => { futures.push(spawn(cube   (*position, *size))) },
                    Sphere { position, radius } => { futures.push(spawn(sphere (*position, *radius, 32, 18))) }
                },
                Object::Camera(_) => { camera_option = Some(object.clone()); }
            }
        }

        let     results  = join_all(futures).await;
        let     camera   = camera_option.map_or(Camera::DEFAULT_3D, |camera| camera);
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

