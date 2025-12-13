// dacho/examples/usage/src/main.rs

use dacho::app::{App, GameTrait};
use dacho::renderer::{InstanceHandle, Meshes, MeshesCapacities, Renderer};
use dacho::renderer::mesh::{Quad, VERTEX_SIZE, INDEX_SIZE, INSTANCE_SIZE};


fn main() {
    App::<Game>::default()
        .run();
}

#[derive(Default)]
struct Game {
    player: Player
}

impl GameTrait for Game {
    fn setup(&mut self) -> Meshes {
        let mut meshes = Meshes::with_capacities(
            &MeshesCapacities {
                different_meshes_count: 1,
                vertex_buffer_size:     1 * 4 *   VERTEX_SIZE,
                index_buffer_size:      6 *        INDEX_SIZE,
                instance_buffer_size:   1 *     INSTANCE_SIZE
            }
        );

        meshes.register::<Quad>(1);

        self.player.handle = Some(
            meshes.add_instance::<Quad>([0.0, 0.0])
        );

        meshes
    }

    fn update(&mut self, renderer: &mut Renderer, delta_time: f32) {
        let Some(player_handle) = self.player.handle.as_ref() else { return; };

        lerp(
            &mut self.player.position,
            self.player.target_position,
            delta_time * 2.0
        );

        renderer.update_instance(player_handle, self.player.position);
    }

    fn cursor(&mut self, x: f64, y: f64) {
        let mut x = x as f32;
        let mut y = y as f32;

        x /= 800.0; // window width  (not after u resize cuz im lazy :D)
        y /= 600.0; // window height
        x -=   0.5;
        y -=   0.5;

        self.player.target_position = [x, y];
    }
}

#[derive(Default)]
struct Player {
    handle:          Option<InstanceHandle>,
    position:        [f32; 2],
    target_position: [f32; 2]
}

fn lerp(a: &mut [f32; 2], b: [f32; 2], t: f32) {
    a[0] = (b[0] - a[0]).mul_add(t, a[0]);
    a[1] = (b[1] - a[1]).mul_add(t, a[1]);
}

