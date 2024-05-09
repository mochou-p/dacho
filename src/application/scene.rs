// dacho/src/application/scene.rs

use {
    anyhow::{Context, Result},
    ash::vk,
    tokio::spawn
};

use {
    super::logger::Logger,
    crate::{
        renderer::geometry::GeometryData,
        log
    }
};

pub struct Scene;

impl Scene {
    pub async fn demo() -> Result<(Vec<GeometryData>, Vec<u8>)> {
        #[cfg(debug_assertions)]
        log!(info, "Loading and generating loading Scene");

        let skybox   = spawn(Self::demo_skybox("spree_bank.jpg"));
        let sphere   = spawn(Self::demo_sphere());
        let light    = spawn(Self::demo_light());
        let vignette = spawn(Self::demo_vignette());

        let (  skybox_g, skybox_t) = skybox   .await??;
        let    sphere_g            = sphere   .await??;
        let     light_g            = light    .await??;
        let  vignette_g            = vignette .await??;

        let scene = vec![
            skybox_g,
            sphere_g,
            light_g,
            vignette_g
        ];

        Ok((scene, skybox_t))
    }

    async fn demo_skybox(filename: &str) -> Result<(GeometryData, Vec<u8>)> {
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

    async fn demo_sphere() -> Result<GeometryData> {
        let (gltf, buffers, _) = gltf::import("assets/models/sphere.glb")?;

        let mut vertices: Vec<f32> = vec![];
        let mut indices:  Vec<u32> = vec![];

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let positions: Vec<[f32; 3]> = reader
                    .read_positions()
                    .context("No glTF positions")?
                    .collect();

                let normals: Vec<[f32; 3]> = reader
                    .read_normals()
                    .context("No glTF normals")?
                    .collect();

                vertices.reserve_exact(positions.len() * 3 + normals.len() * 3);

                for i in 0..positions.len() {
                    vertices.extend_from_slice(&positions[i]);
                    vertices.extend_from_slice(  &normals[i]);
                }

                indices = reader
                    .read_indices()
                    .context("No gltf indices")?
                    .into_u32()
                    .collect();
            }
        }

        let steps = 10;
        let mut instances: Vec<f32> = Vec::with_capacity(steps * steps * 2);

        for y in 0..steps {
            for x in 0..steps {
                instances.extend_from_slice(
                    &[
                        (y as f32/(steps-1) as f32 - 0.5)  * (steps-1) as f32,
                        (-0.5 + x as f32/(steps-1) as f32) * (steps-1) as f32
                    ]
                );
            }
        }

        let shader       = String::from("pbr");
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

        Ok(geometry_data)
    }

    async fn demo_light() -> Result<GeometryData> {
        let (gltf, buffers, _) = gltf::import("assets/models/sphere.glb")?;

        let mut vertices: Vec<f32> = vec![];
        let mut indices:  Vec<u32> = vec![];

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                vertices = reader
                    .read_positions()
                    .context("No glTF positions")?
                    .flatten()
                    .collect();

                indices = reader
                    .read_indices()
                    .context("No gltf indices")?
                    .into_u32()
                    .collect();
            }
        }

        let instances: Vec<f32> = vec![0.0];

        let shader       = String::from("light");
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

        Ok(geometry_data)
    }

    async fn demo_vignette() -> Result<GeometryData> {
        let vertices: Vec<f32> = vec![
            -1.0, -1.0,
             1.0, -1.0,
             1.0,  1.0,
            -1.0,  1.0
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2,  2, 3, 0
        ];

        let instances: Vec<f32> = vec![0.0];

        let shader       = String::from("vignette");
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

