// dacho/examples/usage/src/main.rs

use dacho::app::{App, GameTrait};
use dacho::renderer::{InstanceHandle, Meshes, Renderer};
use dacho::renderer::mesh::{Mesh, Vertex, Index, VERTEX_SIZE, INDEX_SIZE, INSTANCE_SIZE};


fn main() {
    App::<Game>::default()
        .run();
}

#[derive(Default)]
struct Game {
    dancer: Option<InstanceHandle>,
    z:      f32
}

impl GameTrait for Game {
    fn setup(&mut self) -> Option<Meshes> {
        let mut meshes = Meshes::with_capacities(
            1,
            3 * VERTEX_SIZE,
            INDEX_SIZE,
            INSTANCE_SIZE
        );

        meshes.register::<MyTriangle>(1);

        self.dancer = Some(meshes.add_instance::<MyTriangle>([0.0, 0.0]));

        Some(meshes)
    }

    fn update(&mut self, renderer: &mut Renderer) {
        let dancer = self.dancer.as_ref().unwrap();

        let x = self.z.cos() * 0.5;
        let y = self.z.sin() * 0.5;

        self.z += 0.00002;

        renderer.update_instance(dancer, [x, y]);
    }
}

struct MyTriangle;
impl Mesh for MyTriangle {
    fn vertices() -> impl IntoIterator<Item = Vertex> {
        lets_pretend_this_loads_a_model()
    }

    fn indices() -> impl IntoIterator<Item = Index> {
        [
            [0, 1, 2]
        ]
    }
}

fn lets_pretend_this_loads_a_model() -> Vec<[f32; VERTEX_SIZE]> {
    vec![
        [ 0.00, -0.04],
        [-0.05,  0.04],
        [ 0.05,  0.04]
    ]
}

