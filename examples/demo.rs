// dacho/examples/demo.rs

use dacho::prelude::*;

fn main() {
    World::new()
        .add(&[
            Cube::default()
                .size(V3::new(5.0, 0.4, 5.0))
                .anchor(Anchor::Top)
                .build(),
            Sphere::default()
                .color(Color::BLUE)
                .material(Material::METAL)
                .anchor(Anchor::Bottom)
                .build()
        ])
        .run();
}

