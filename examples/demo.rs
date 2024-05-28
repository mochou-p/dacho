// dacho/examples/demo.rs

use dacho::prelude::*;

fn main() {
    World::new()
        .add(&[
            Cube::default()
                .position(V3::Y * -0.2)
                .size(V3::new(5.0, 0.4, 5.0))
                .build(),
            Sphere::default()
                .position(V3::Y * 0.5)
                .color(Color::BLUE)
                .material(Material::METAL)
                .build()
        ])
        .run();
}

