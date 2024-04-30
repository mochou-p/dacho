// dacho/src/application/scene.rs

use anyhow::{Context, Result};

use ash::vk;

#[cfg(debug_assertions)]
use super::logger::Logger;

use crate::renderer::geometry::GeometryData;

pub struct Scene;

impl Scene {
    #[allow(clippy::type_complexity)]
    pub fn demo() -> Result<(Vec<GeometryData>, Vec<Vec<u8>>, Vec<Vec<u8>>)> {
        #[cfg(debug_assertions)]
        Logger::info("Loading and generating loading Scene");

        let (sky, cubemap)    = Self::demo_skybox("nature")?;
        let (model, textures) = Self::demo_gltf("damaged_helmet")?;

        let scene = vec![
            sky,
            model,
            Self::demo_vignette()?
        ];

        Ok((scene, cubemap, textures))
    }

    fn demo_skybox(filename: &str) -> Result<(GeometryData, Vec<Vec<u8>>)> {
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

        let mut cubemap: Vec<Vec<u8>> = vec![];

        let faces = [
            "nx",
            "px",
            "ny",
            "py",
            "pz",
            "nz"
        ];

        let mut axis_size = 0;

        for face in faces.iter() {
            let image = image::io::Reader::open(
                format!("assets/textures/skybox/{filename}/{face}.png")
            )?.decode()?;

            let image_data = image
                .as_rgba8()
                .context("Failed to cast Cubemap to R8G8B8A8")?;

            let (width, height) = image_data.dimensions();

            if width != height {
                panic!("Cubemap face is not square");
            }

            if axis_size == 0 {
                axis_size = width
            } else if width != axis_size {
                panic!("Cubemap faces do not share dimensions");
            }

            let mut pixels: Vec<u8> = vec![];

            for pixel in image_data.pixels() {
                pixels.extend_from_slice(&pixel.0);
            }

            if pixels.len() as u32 != width * height * 4 {
                println!("Pixel count error");
            }

            cubemap.push(pixels);
        }

        if cubemap.len() != 6 {
            panic!("Cubemap reading error");
        }

        let shader       = String::from("skybox");
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

        Ok((geometry_data, cubemap))
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
                    .context("No glTF positions")?
                    .collect();

                let normals: Vec<[f32; 3]> = reader
                    .read_normals()
                    .context("No glTF normals")?
                    .collect();

                let tex_coords: Vec<[f32; 2]> = reader
                    .read_tex_coords(0)
                    .context("No glTF texture coordinates")?
                    .into_f32()
                    .collect();

                if !(
                    positions.len() ==    normals.len() &&
                    positions.len() == tex_coords.len()
                ) {
                    panic!("glTF vertex input error");
                }

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
            let image_indices = [
                material
                    .pbr_metallic_roughness()
                    .base_color_texture()
                    .context("No glTF base color")?
                    .texture()
                    .index(),
                material
                    .normal_texture()
                    .context("No glTF normal map")?
                    .texture()
                    .index(),
                material
                    .pbr_metallic_roughness()
                    .metallic_roughness_texture()
                    .context("No glTF metallic roughness")?
                    .texture()
                    .index(),
                material
                    .emissive_texture()
                    .context("No glTF emission")?
                    .texture()
                    .index(),
                material
                    .occlusion_texture()
                    .context("No glTF occlusion")?
                    .texture()
                    .index()
            ];

            for i in image_indices {
                if images[i].format != gltf::image::Format::R8G8B8 {
                    panic!("Only gltf::image::Format::R8G8B8 is supported");
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

        let instances: Vec<f32> = vec![0.0];

        let shader       = String::from("pbr");
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

