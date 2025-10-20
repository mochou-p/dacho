// dacho/examples/usage/src/main.rs

use dacho::app::{App, GameTrait};
use dacho::renderer::{Meshes, Circle, Quad};


fn main() {
    App::<Game>::default()
        .run();
}

#[derive(Default)]
struct Game;

impl GameTrait for Game {
    fn setup(&mut self) -> Option<Meshes> {
        let mut meshes = Meshes::with_size_estimates(2, 32, 32, 128);

        meshes.register::<Quad>  (4);
        meshes.register::<Circle>(4);

        meshes.add_instance::<Quad>  ([ 0.0, -0.5]);
        meshes.add_instance::<Quad>  ([ 0.0,  0.5]);
        meshes.add_instance::<Quad>  ([ 0.5, -0.5]);
        meshes.add_instance::<Quad>  ([ 0.5,  0.5]);
        meshes.add_instance::<Circle>([-0.5,  0.0]);
        meshes.add_instance::<Circle>([-0.5, -0.5]);
        meshes.add_instance::<Circle>([-0.5,  0.5]);
        meshes.add_instance::<Circle>([ 0.5,  0.0]);

        Some(meshes)
    }
}

