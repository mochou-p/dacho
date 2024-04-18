// dacho/src/application/scene.rs

use ash::vk;

use crate::renderer::{
    geometry::GeometryData,
    vertex_input::Type
};

pub struct Scene;

impl Scene {
    pub fn demo() -> Vec<GeometryData> {
        vec![
            Self::demo_tiles(),
            Self::demo_grass()
        ]
    }

    fn demo_tiles() -> GeometryData {
        let   vertex_info = vec![Type::Vec4];
        let instance_info = vec![Type::Vec3];

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

        let i        = 5;
        let offset   = (i - 1) as f32 * 0.5;

        for z in 0..i {
            for x in 0..i {
                instances.push(grid_size * (x as f32 - offset));
                instances.push(0.0);
                instances.push(grid_size * (z as f32 - offset));
            }
        }

        let shader            = String::from("tile");
        let cull_mode         = vk::CullModeFlags::BACK;
        let descriptor_set_id = Some(0);

        GeometryData::new(
            shader,
            cull_mode,
            descriptor_set_id,
            vertex_info,
            instance_info,
            vertices,
            instances,
            indices
        )
    }

    fn demo_grass() -> GeometryData {
        let   vertex_info = vec![Type::Vec3];
        let instance_info = vec![Type::Vec3];

        let vertices: Vec<f32> = vec![
             0.00, 4.0, 0.0,
             0.08, 2.4, 0.0,
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

        let grid_size = 16.0;
        let i         = 2;
        let offset1   = grid_size / i as f32;
        let offset2   = (i - 1) as f32 * 0.5;

        for z in 0..i {
            for x in 0..i {
               instances.push(offset1 * (x as f32 - offset2));
               instances.push(0.0);
               instances.push(offset1 * (z as f32 - offset2));
            }
        }

        let shader            = String::from("grass");
        let cull_mode         = vk::CullModeFlags::NONE;
        let descriptor_set_id = None;

        GeometryData::new(
            shader,
            cull_mode,
            descriptor_set_id,
            vertex_info,
            instance_info,
            vertices,
            instances,
            indices
        )
    }
}

