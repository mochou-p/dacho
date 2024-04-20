// dacho/src/application/scene.rs

use anyhow::Result;

use ash::vk;

use crate::renderer::geometry::GeometryData;

pub struct Scene;

impl Scene {
    pub fn demo() -> Result<Vec<GeometryData>> {
        let scene = vec![
            Self::demo_skybox()?,
            Self::demo_tiles()?,
            Self::demo_grass()?,
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

        let indices: Vec<u16> = vec![
            0, 1, 2, 2, 3, 0,
            7, 6, 5, 5, 4, 7,
            4, 5, 1, 1, 0, 4,
            6, 7, 3, 3, 2, 6,
            0, 3, 7, 7, 4, 0,
            2, 1, 5, 5, 6, 2
        ];

        let instances: Vec<f32> = vec![
            0.0, 0.0, 0.0
        ];

        let shader    = String::from("sky");
        let cull_mode = vk::CullModeFlags::FRONT;

        let geometry_data = GeometryData::new(
            shader,
            cull_mode,
            vertices,
            instances,
            indices
        )?;

        Ok(geometry_data)
    }

    fn demo_tiles() -> Result<GeometryData> {
        let grid_size = 16.0;
        let grid_half = grid_size * 0.5;
        let step_frac = 1.0 / grid_size;

        let vertices: Vec<f32> = vec![
            -grid_half, 0.0, -grid_half, step_frac,
             grid_half, 0.0, -grid_half, step_frac,
             grid_half, 0.0,  grid_half, step_frac,
            -grid_half, 0.0,  grid_half, step_frac
        ];

        let indices: Vec<u16> = vec![
            0, 1, 2,
            2, 3, 0
        ];

        let mut instances: Vec<f32> = vec![];

        let i      = 15;
        let offset = (i - 1) as f32 * 0.5;

        for z in 0..i {
            for x in 0..i {
                instances.push(grid_size * (x as f32 - offset));
                instances.push(0.0);
                instances.push(grid_size * (z as f32 - offset));
            }
        }

        let shader    = String::from("tile");
        let cull_mode = vk::CullModeFlags::BACK;

        let geometry_data = GeometryData::new(
            shader,
            cull_mode,
            vertices,
            instances,
            indices
        )?;

        Ok(geometry_data)
    }

    fn demo_grass() -> Result<GeometryData> {
        let vertices: Vec<f32> = vec![
             0.00, 4.0, 0.0,
             0.08, 1.8, 0.0,
             0.18, 0.0, 0.0,
            -0.18, 0.0, 0.0,
            -0.08, 1.8, 0.0
        ];

        let indices: Vec<u16> = vec![
            0, 1, 4,
            1, 2, 3,
            1, 3, 4
        ];

        let mut instances: Vec<f32> = vec![];

        let grid_i    = (15 - 1) / 2;
        let grid_size = 16.0;
        let i         = 32;
        let offset1   = grid_size / i as f32;
        let offset2   = (i - 1) as f32 * 0.5;

        for chunk_z in -grid_i..grid_i + 1 {
            for chunk_x in -grid_i..grid_i + 1 {
                for z in 0..i {
                    for x in 0..i {
                        let x_   = x as f32 - offset2;
                        let z_   = z as f32 - offset2;

                        let x__  = offset1 * x_ + noise(x_, z_ + chunk_x as f32) * offset1 * 0.5;
                        let z__  = offset1 * z_ + noise(z_, x_ + chunk_z as f32) * offset1 * 0.5;

                        let x___ = chunk_x as f32 * grid_size;
                        let z___ = chunk_z as f32 * grid_size;

                        instances.push(x__ + x___);
                        instances.push(0.0);
                        instances.push(z__ + z___);
                    }
                }
            }
        }

        let shader    = String::from("grass");
        let cull_mode = vk::CullModeFlags::NONE;

        let geometry_data = GeometryData::new(
            shader,
            cull_mode,
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

        let indices: Vec<u16> = vec![
            0, 1, 2, 2, 3, 0
        ];

        let instances: Vec<f32> = vec![
            0.0, 0.0, 0.0
        ];

        let shader    = String::from("vignette");
        let cull_mode = vk::CullModeFlags::BACK;

        let geometry_data = GeometryData::new(
            shader,
            cull_mode,
            vertices,
            instances,
            indices
        )?;

        Ok(geometry_data)
    }
}

fn noise(x: f32, y: f32) -> f32 {
    (glam::Vec2::new(x, y).dot(glam::Vec2::new(12.9898, 4.1414)).sin() * 43758.5453) % 1.0
}

