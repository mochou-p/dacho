// dacho/src/application/scene.rs

use anyhow::{Context, Result};

use ash::vk;

#[cfg(debug_assertions)]
use super::logger::Logger;

use crate::renderer::geometry::GeometryData;

pub struct Scene;

impl Scene {
    pub fn demo() -> Result<(Vec<GeometryData>, Vec<Vec<u8>>)> {
        #[cfg(debug_assertions)]
        Logger::info("Loading and generating loading Scene");

        let ground_size = 128.0;

        let (model, textures) = Self::demo_gltf("damaged_helmet")?;

        let scene = vec![
            Self::demo_skybox()?,
            Self::demo_ground(ground_size)?,
            Self::demo_grass(ground_size)?,
            model,
            Self::demo_vignette()?
        ];

        Ok((scene, textures))
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
            0, 1, 2,  2, 3, 0,
            7, 6, 5,  5, 4, 7,
            4, 5, 1,  1, 0, 4,
            6, 7, 3,  3, 2, 6,
            0, 3, 7,  7, 4, 0,
            2, 1, 5,  5, 6, 2
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
            -0.08, 1.8
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

    fn demo_gltf(filename: &str) -> Result<(GeometryData, Vec<Vec<u8>>)> {
        let (gltf, buffers, images) = gltf::import(format!("assets/models/{filename}.glb"))?;

        let mut vertices: Vec<f32> = vec![];
        let mut indices:  Vec<u32> = vec![];

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                let positions: Vec<[f32; 3]> = reader
                    .read_positions()
                    .context("No gltf positions")?
                    .collect();

                let normals: Vec<[f32; 3]> = reader
                    .read_normals()
                    .context("No gltf normals")?
                    .collect();

                let tex_coords: Vec<[f32; 2]> = reader
                    .read_tex_coords(0)
                    .context("No gltf texture coordinates")?
                    .into_f32()
                    .collect();

                for i in 0..positions.len() {
                    vertices.extend_from_slice( &positions[i]);
                    vertices.extend_from_slice(   &normals[i]);
                    vertices.extend_from_slice(&tex_coords[i]);
                }

                indices = reader
                    .read_indices()
                    .context("No gltf indices")?
                    .into_u32()
                    .collect();
            }
        }

        let mut textures: Vec<Vec<u8>> = vec![];

        for material in gltf.materials() {
            let is = [
                material
                    .pbr_metallic_roughness()
                    .base_color_texture()
                    .context("No glTF base color texture")?
                    .texture()
                    .index(),
                material
                    .normal_texture()
                    .context("No glTF normal texture")?
                    .texture()
                    .index()
            ];

            for i in is {
                if images[i].format != gltf::image::Format::R8G8B8 {
                    panic!("Unsupported glTF image format");
                }

                if images[i].width != images[i].height {
                    panic!("glTF image dimensions do not match");
                }

                if (images[i].pixels.len() % 3) != 0 {
                    panic!("glTF image pixel data error");
                }

                let mut pixels: Vec<u8> = vec![];

                for j in (0..(images[i].pixels.len())).step_by(3) {
                    pixels.extend_from_slice(
                        &[
                            images[i].pixels[j],
                            images[i].pixels[j + 1],
                            images[i].pixels[j + 2],
                            255
                        ]
                    );
                }

                if pixels.is_empty() {
                    panic!("glTF image pixel reading error");
                }

                textures.push(pixels);
            }
        }

        if textures.is_empty() {
            panic!("glTF textures missing");
        }

        let instances: Vec<f32> = vec![5.0];

        let shader       = String::from("test");
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

        Ok((geometry_data, textures))
    }

    fn demo_vignette() -> Result<GeometryData> {
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

