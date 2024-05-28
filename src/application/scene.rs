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
            // position                           normal
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
        const SECTORS: usize = 32;
        const STACKS:  usize = 18;

        // * 3 -> xyz
        // * 2 -> position, normal
        let mut vertices: Vec<f32> = Vec::with_capacity((SECTORS + 1) * (STACKS + 1) * 3 * 2);

        // * 6 -> indices per quad
        let mut indices:  Vec<u32> = Vec::with_capacity(SECTORS * (STACKS - 1) * 6);

        let sector_step = 2.0 * std::f32::consts::PI / SECTORS as f32;
        let stack_step  = std::f32::consts::PI / STACKS as f32;

        for i in 0..STACKS + 1 {
            let a  = std::f32::consts::FRAC_PI_2 - (i as f32) * stack_step;
            let xy = a.cos();
            let z  = a.sin();

            for j in 0..SECTORS + 1 {
                let a = (j as f32) * sector_step;
                let x = xy * a.cos();
                let y = xy * a.sin();

                vertices.extend_from_slice(
                    &[
                        // position
                        x * radius + position.x,
                        y * radius - position.y,
                        z * radius + position.z,

                        // normal
                        x,
                        y,
                        z
                    ]
                );
            }
        }

        for i in 0..STACKS {
            let mut k1 = (i * (SECTORS + 1)) as u32;
            let mut k2 = k1 + SECTORS as u32 + 1;

            for _j in 0..SECTORS {
                if i != 0 {
                    indices.push(k1 + 1);
                    indices.push(k2);
                    indices.push(k1);
                }

                if i != STACKS - 1 {
                    indices.push(k2 + 1);
                    indices.push(k2);
                    indices.push(k1 + 1);
                }

                k1 += 1;
                k2 += 1;
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

