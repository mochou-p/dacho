// dacho/src/application/scene.rs

use crate::renderer::{
    geometry::GeometryData,
    vertex_input::{instance::Instance, vertex::Vertex}
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
        let grid_size = 16.0;
        let grid_half = grid_size * 0.5;
        let step_frac = 1.0 / grid_size;

        let vertices = vec![
            Vertex::new(-grid_half, 0.0, -grid_half, step_frac),
            Vertex::new( grid_half, 0.0, -grid_half, step_frac),
            Vertex::new( grid_half, 0.0,  grid_half, step_frac),
            Vertex::new(-grid_half, 0.0,  grid_half, step_frac)
        ];

        let indices: Vec<u16> = vec![
            0, 1, 2,
            2, 3, 0
        ];

        let mut instances = vec![];

        let i      = 2_usize.pow(8) - 1;
        let offset = (i - 1) as f32 * 0.5;

        for z in 0..i {
            for x in 0..i {
                instances.push(
                    Instance::new(
                        grid_size * (x as f32 - offset),
                        0.0,
                        grid_size * (z as f32 - offset)
                    )
                );
            }
        }

        let pipeline_id       = Some(0);
        let descriptor_set_id = Some(0);

        GeometryData::new(
            pipeline_id,
            descriptor_set_id,
            vertices,
            instances,
            indices
        )
    }

    fn demo_grass() -> GeometryData {
        let w = 0.0;

        let vertices = vec![
            Vertex::new( 0.00, 4.0, 0.0, w),
            Vertex::new( 0.08, 2.4, 0.0, w),
            Vertex::new( 0.18, 0.0, 0.0, w),
            Vertex::new(-0.18, 0.0, 0.0, w),
            Vertex::new(-0.08, 1.8, 0.0, w),
        ];

        let indices = vec![
            0, 1, 4,
            1, 2, 3,
            1, 3, 4
        ];

        let mut instances = vec![];

        let grid_size = 16.0;
        let i         = 32;
        let offset1   = grid_size / i as f32;
        let offset2   = (i - 1) as f32 * 0.5;

        for z in 0..i {
            for x in 0..i {
                instances.push(
                    Instance::new(
                        offset1 * (x as f32 - offset2),
                        0.0,
                        offset1 * (z as f32 - offset2)
                    )
                );
            }
        }

        let pipeline_id       = Some(1);
        let descriptor_set_id = None;

        GeometryData::new(
            pipeline_id,
            descriptor_set_id,
            vertices,
            instances,
            indices
        )
    }
}

