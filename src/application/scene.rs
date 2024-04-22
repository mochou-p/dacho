// dacho/src/application/scene.rs

use anyhow::{Context, Result};

use ash::vk;

#[cfg(debug_assertions)]
use super::logger::Logger;

use crate::renderer::geometry::GeometryData;

pub struct Scene;

impl Scene {
    pub fn demo() -> Result<Vec<GeometryData>> {
        #[cfg(debug_assertions)]
        Logger::info("Loading and generating loading Scene");

        let ground_size = 128.0;

        let scene = vec![
            Self::demo_skybox()?,
            Self::demo_ground(ground_size)?,
            Self::demo_grass(ground_size)?,
            Self::demo_helmet()?,
            Self::demo_vignette()?
        ];


        Ok(scene)
    }

    fn demo_skybox() -> Result<GeometryData> {
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
            0, 1, 2, 2, 3, 0,
            7, 6, 5, 5, 4, 7,
            4, 5, 1, 1, 0, 4,
            6, 7, 3, 3, 2, 6,
            0, 3, 7, 7, 4, 0,
            2, 1, 5, 5, 6, 2
        ];

        let instances: Vec<f32> = vec![0.0];

        let shader       = String::from("sky");
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

    fn demo_ground(size: f32) -> Result<GeometryData> {
        let half = size * 0.5;

        let vertices: Vec<f32> = vec![
            -half, -half,
             half, -half,
             half,  half,
            -half,  half
        ];

        let indices: Vec<u32> = vec![
            0, 1, 3, 2
        ];

        let instances: Vec<f32> = vec![0.0];

        let shader       = String::from("ground");
        let cull_mode    = vk::CullModeFlags::NONE;
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

    fn demo_grass(ground_size: f32) -> Result<GeometryData> {
        let vertices: Vec<f32> = vec![
             0.00, 4.0,
             0.08, 1.8,
             0.18, 0.0,
            -0.18, 0.0,
            -0.08, 1.8,
        ];

        let indices: Vec<u32> = vec![
            0, 1, 4,
            1, 2, 3,
            1, 3, 4
        ];

        let mut instances: Vec<f32> = vec![];

        let i = ground_size as usize;
        let o = (i - 1) as f32 * 0.5;

        let grass_per_unit = 3;

        for z in 0..i {
            for x in 0..i {
                for w in 0..grass_per_unit {
                    let x_ = x as f32 + w as f32 * 1.0 / grass_per_unit as f32;
                    let z_ = z as f32 + w as f32 * 1.0 / grass_per_unit as f32;

                    instances.push((x_ - o) + noise(x_, z_) * 0.5);
                    instances.push((z_ - o) + noise(z_, x_) * 0.5);
                }
            }
        }

        let shader       = String::from("grass");
        let cull_mode    = vk::CullModeFlags::NONE;
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

    fn demo_helmet() -> Result<GeometryData> {
        let (gltf, buffers, _) = gltf::import("assets/models/damaged_helmet.glb")?;

        let mut vertices: Vec<f32> = vec![];
        let mut indices:  Vec<u32> = vec![];

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                vertices = reader
                    .read_positions()
                    .context("No gltf positions")?
                    .flat_map(|vertex| vertex.to_vec())
                    .collect();

                indices = reader
                    .read_indices()
                    .context("No gltf indices")?
                    .into_u32()
                    .collect();
            }
        }

        let instances: Vec<f32> = vec![3.0];

        let shader       = String::from("test");
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

    fn demo_vignette() -> Result<GeometryData> {
        let vertices: Vec<f32> = vec![
            -1.0, -1.0,
             1.0, -1.0,
             1.0,  1.0,
            -1.0,  1.0
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2, 2, 3, 0
        ];

        let instances: Vec<f32> = vec![0.0];

        let shader       = String::from("vignette");
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
}

fn noise(x: f32, y: f32) -> f32 {
    (glam::Vec2::new(x, y).dot(glam::Vec2::new(12.9898, 4.1414)).sin() * 43758.545) % 1.0
}

