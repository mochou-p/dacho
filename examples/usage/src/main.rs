// dacho/examples/usage/src/main.rs

use dacho::app::{App, GameTrait};
use dacho::renderer::{InstanceHandle, Mesh, Meshes, Renderer, Circle, Quad};
use dacho::renderer::{VERTEX_SIZE, INDEX_SIZE};


fn main() {
    App::<Game>::default()
        .run();
}

fn lets_pretend_this_loads_a_model() -> Vec<[f32; VERTEX_SIZE]> {
    vec![
        [ 0.00, -0.04],
        [-0.05,  0.04],
        [ 0.05,  0.04]
    ]
}

struct MyTriangle;
impl Mesh for MyTriangle {
    fn vertices() -> Vec<[f32; VERTEX_SIZE]> {
        lets_pretend_this_loads_a_model()
    }

    fn indices() -> Vec<[u32; INDEX_SIZE]> {
        vec![
            [0, 1, 2]
        ]
    }
}

#[derive(Default)]
struct Game {
    dancer:  InstanceHandle,
    z:       f32
}

impl GameTrait for Game {
    fn setup(&mut self) -> Option<Meshes> {
        let mut meshes = Meshes::with_size_estimates(3, 64, 64, 256);

        meshes.register::<Circle>    (2);
        meshes.register::<Quad>      (3);
        meshes.register::<MyTriangle>(1);

        meshes.add_instance::<Circle>    ([-0.7, -0.7]);
        meshes.add_instance::<Circle>    ([ 0.7,  0.7]);
        meshes.add_instance::<Quad>      ([-0.5, -0.4]);
        meshes.add_instance::<Quad>      ([ 0.4,  0.4]);
        meshes.add_instance::<Quad>      ([ 0.4,  0.4]);
        meshes.add_instance::<MyTriangle>([ 0.0,  0.0]);

        self.dancer = meshes.add_instance::<Circle>([0.0, 0.0]);

        Some(meshes)
    }

    fn update(&mut self, renderer: &mut Renderer) {
        self.z += 0.00003;

        let x = self.z.cos() * 0.25;
        let y = self.z.sin() * 0.25;

        renderer.update_instance(&self.dancer, [x, y]);
    }
}

