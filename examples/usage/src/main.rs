// dacho/examples/usage/src/main.rs

use dacho::app::{App, GameTrait};
use dacho::renderer::{InstanceHandle, Meshes, Renderer, Circle, Quad};


fn main() {
    App::<Game>::default()
        .run();
}

#[derive(Default)]
struct Game {
    dancer:  InstanceHandle,
    z:       f32
}

impl GameTrait for Game {
    fn setup(&mut self) -> Option<Meshes> {
        let mut meshes = Meshes::with_size_estimates(2, 32, 32, 128);

        meshes.register::<Circle>(2);
        meshes.register::<Quad>  (3);

        meshes.add_instance::<Circle>([-0.7, -0.7]);
        meshes.add_instance::<Circle>([ 0.7,  0.7]);
        meshes.add_instance::<Quad>  ([-0.5, -0.4]);
        meshes.add_instance::<Quad>  ([ 0.4,  0.4]);

        self.dancer = meshes.add_instance::<Circle>([0.0, 0.0]);

        Some(meshes)
    }

    fn update(&mut self, renderer: &mut Renderer) {
        self.z += 0.00005;

        let x = self.z.cos() * 0.1;
        let y = self.z.sin() * 0.1;

        renderer.update_instance(&self.dancer, [x, y]);
    }
}

