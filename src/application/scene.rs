// dacho/src/application/scene.rs

use {
    anyhow::{Context, Result},
    ash::vk,
    futures::future::join_all,
    serde::{Serialize, Deserialize},
    tokio::spawn
};

use {
    super::logger::Logger,
    crate::{
        prelude::{
            colors::Color,
            materials::Material,
            primitives::{cube, sphere},
            shapes::Object::{Cube, Sphere},
            types::V3,
            world::World
        },
        renderer::rendering::geometry::*,
        log
    }
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub geometry: Vec<GeometryData>,
    pub texture:  Vec<u8>
}

impl Data {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { geometry: vec![], texture: vec![] }
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

        let skybox = spawn(Self::skybox("evening.jpg"));
        let light  = spawn(sphere(V3::ZERO, 0.03, Color::BLACK, Material::ROUGH, 16, 9, "light"));

        let (skybox_g, texture) = skybox.await??;
        let   light_g           = light.await??;

        geometry.push(skybox_g);
        geometry.push( light_g);

        Ok(Data { geometry, texture })
    }

    async fn skybox(filename: &str) -> Result<(GeometryData, Vec<u8>)> {
        let vertices: Vec<f32> = vec![
            -1.0,  1.0, -1.0,
             1.0,  1.0, -1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,
            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0, -1.0,  1.0,
            -1.0, -1.0,  1.0
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2,  2, 3, 0,
            7, 6, 5,  5, 4, 7,
            4, 5, 1,  1, 0, 4,
            6, 7, 3,  3, 2, 6,
            0, 3, 7,  7, 4, 0,
            2, 1, 5,  5, 6, 2
        ];

        let instances: Vec<f32> = vec![0.0];

        let image = image::io::Reader::open(
            format!("assets/textures/skybox/{filename}")
        )?.decode()?;

        let image_data = image
            .as_rgb8()
            .context("Failed to cast Skybox image to R8G8B8")?;

        let (width, height) = image_data.dimensions();

        if width != height * 2 {
            log!(panic, "Skybox is not spherical");
        }

        let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

        for pixel in image_data.pixels() {
            let pixel = &pixel.0;
            pixels.extend_from_slice(
                &[pixel[0], pixel[1], pixel[2], 255]
            );
        }

        let shader       = String::from("skybox");
        let cull_mode    = vk::CullModeFlags::BACK;
        let polygon_mode = vk::PolygonMode::FILL;

        let geometry_data = GeometryData::new(
            shader,
            cull_mode,
            polygon_mode,
            vertices,
            instances,
            indices
        )?;

        Ok((geometry_data, pixels))
    }
}

