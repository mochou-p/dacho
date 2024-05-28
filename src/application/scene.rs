// dacho/src/application/scene.rs

use {
    anyhow::{Context, Result},
    ash::vk,
    futures::future::join_all,
    tokio::spawn
};

use {
    super::logger::Logger,
    crate::{
        prelude::{
            world::World,
            Cube, Sphere, V2, V3
        },
        renderer::geometry::GeometryData,
        log
    }
};

pub struct Scene;

impl Scene {
    pub async fn build(world: &World) -> Result<(Vec<GeometryData>, Vec<u8>)> {
        #[cfg(debug_assertions)]
        log!(info, "Building Scene");

        let mut futures = vec![];

        for object in world.objects.iter() {
            futures.push(
                match object {
                    Cube   (p, s, c, m) => { spawn(Self::cube   (*p, *s, *c, *m)) },
                    Sphere (p, s, c, m) => { spawn(Self::sphere (*p, *s, *c, *m)) }
                }
            );
        }

        let results = join_all(futures).await;

        let mut scene = vec![];

        for object in results.iter() {
            match object {
                Ok(result) => match result {
                    Ok(result) => { scene.push(result.clone()); },
                    Err(err)   => { log!(panic, "{err}"); panic!(); }
                },
                Err(err) => { log!(panic, "{err}"); panic!(); }
            }
        }

        let  skybox              = spawn(Self::skybox("evening.jpg"));
        let (skybox_g, skybox_t) = skybox.await??;

        scene.push(skybox_g);

        Ok((scene, skybox_t))
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

    async fn cube(p: V3, size: V3, color: V3, metrou: V2) -> Result<GeometryData> {
        let hs = size * 0.5;

        let vertices: Vec<f32> = vec![
            p.x - hs.x, -p.y + hs.y, p.z - hs.z,  0.0,  1.0,  0.0,
            p.x + hs.x, -p.y + hs.y, p.z - hs.z,  0.0,  1.0,  0.0,
            p.x + hs.x, -p.y + hs.y, p.z + hs.z,  0.0,  1.0,  0.0,
            p.x - hs.x, -p.y + hs.y, p.z + hs.z,  0.0,  1.0,  0.0,

            p.x - hs.x, -p.y - hs.y, p.z - hs.z,  0.0, -1.0,  0.0,
            p.x + hs.x, -p.y - hs.y, p.z - hs.z,  0.0, -1.0,  0.0,
            p.x + hs.x, -p.y - hs.y, p.z + hs.z,  0.0, -1.0,  0.0,
            p.x - hs.x, -p.y - hs.y, p.z + hs.z,  0.0, -1.0,  0.0,

            p.x - hs.x, -p.y - hs.y, p.z - hs.z, -1.0,  0.0,  0.0,
            p.x - hs.x, -p.y + hs.y, p.z - hs.z, -1.0,  0.0,  0.0,
            p.x - hs.x, -p.y + hs.y, p.z + hs.z, -1.0,  0.0,  0.0,
            p.x - hs.x, -p.y - hs.y, p.z + hs.z, -1.0,  0.0,  0.0,

            p.x + hs.x, -p.y - hs.y, p.z - hs.z,  1.0,  0.0,  0.0,
            p.x + hs.x, -p.y + hs.y, p.z - hs.z,  1.0,  0.0,  0.0,
            p.x + hs.x, -p.y + hs.y, p.z + hs.z,  1.0,  0.0,  0.0,
            p.x + hs.x, -p.y - hs.y, p.z + hs.z,  1.0,  0.0,  0.0,

            p.x - hs.x, -p.y - hs.y, p.z + hs.z,  0.0,  0.0,  1.0,
            p.x + hs.x, -p.y - hs.y, p.z + hs.z,  0.0,  0.0,  1.0,
            p.x + hs.x, -p.y + hs.y, p.z + hs.z,  0.0,  0.0,  1.0,
            p.x - hs.x, -p.y + hs.y, p.z + hs.z,  0.0,  0.0,  1.0,

            p.x - hs.x, -p.y - hs.y, p.z - hs.z,  0.0,  0.0, -1.0,
            p.x + hs.x, -p.y - hs.y, p.z - hs.z,  0.0,  0.0, -1.0,
            p.x + hs.x, -p.y + hs.y, p.z - hs.z,  0.0,  0.0, -1.0,
            p.x - hs.x, -p.y + hs.y, p.z - hs.z,  0.0,  0.0, -1.0
        ];

        let indices: Vec<u32> = vec![
             0,  1,  2,   2,  3,  0,
             7,  6,  5,   5,  4,  7,
             8,  9, 10,  10, 11,  8,
            15, 14, 13,  13, 12, 15,
            19, 18, 17,  17, 16, 19,
            20, 21, 22,  22, 23, 20
        ];

        let instances: Vec<f32> = vec![color.x, color.y, color.z, metrou.x, metrou.y];

        let shader       = String::from("pbr");
        let cull_mode    = vk::CullModeFlags::FRONT;
        let polygon_mode = vk::PolygonMode::FILL;

        let geometry_data = GeometryData::new(
            shader,
            cull_mode,
            polygon_mode,
            vertices,
            instances,
            indices
        )?;

        Ok(geometry_data)
    }

    async fn sphere(position: V3, radius: f32, color: V3, metrou: V2) -> Result<GeometryData> {
        let (gltf, buffers, _) = gltf::import("assets/models/sphere.glb")?;

        let mut vertices: Vec<f32> = vec![];
        let mut indices:  Vec<u32> = vec![];

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                vertices = reader
                    .read_positions()
                    .context("No glTF positions")?
                    .flat_map(|p| {
                        let temp = [-p[0], -p[1], -p[2]];

                        [(V3::from(temp) * radius - position).to_array(), temp]
                    })
                    .flatten()
                    .collect();

                indices = reader
                    .read_indices()
                    .context("No gltf indices")?
                    .into_u32()
                    .collect();
            }
        }

        let instances: Vec<f32> = vec![color.x, color.y, color.z, metrou.x, metrou.y];

        let shader       = String::from("pbr");
        let cull_mode    = vk::CullModeFlags::FRONT;
        let polygon_mode = vk::PolygonMode::FILL;

        let geometry_data = GeometryData::new(
            shader,
            cull_mode,
            polygon_mode,
            vertices,
            instances,
            indices
        )?;

        Ok(geometry_data)
    }
}

