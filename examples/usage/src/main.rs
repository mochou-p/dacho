// dacho/examples/usage/src/main.rs

use dacho::app::{App, GameTrait};
use dacho::renderer::{Meshes, MeshesCapacities};
use dacho::renderer::mesh::{Quad, VERTEX_SIZE, INDEX_SIZE, INSTANCE_SIZE};


fn main() {
    App::<Game>::default()
        .run();
}

#[derive(Default)]
struct Game;

impl GameTrait for Game {
    fn setup(&mut self) -> Meshes {
        let per_w = 16;
        let per_h =  8;
        let count = per_w * per_h;

        let mut meshes = Meshes::with_capacities(
            &MeshesCapacities {
                different_meshes_count: 1,
                vertex_buffer_size:     4     *   VERTEX_SIZE,
                index_buffer_size:      6     *    INDEX_SIZE,
                instance_buffer_size:   count * INSTANCE_SIZE
            }
        );

        meshes.register::<Quad>(count);

        for y in 0..per_h {
            for x in 0..per_w {
                let x = (x as f32 / (per_w - 1) as f32 - 0.5) * 1.7;
                let y =  y as f32 / (per_h - 1) as f32 - 0.5;

                meshes.add_instance::<Quad>([x, y]);
            }
        }

        meshes
    }
}

