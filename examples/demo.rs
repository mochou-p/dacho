// dacho/examples/demo.rs

use dacho::prelude::*;

fn main() {
    World::new()
        .add(&[
            Cube::default()
                .size(V3::new(5.0, 0.4, 5.0))
                .anchor(Anchor::Top)
                .build(),
            Cube::default()
                .position(V3::X)
                .size(V3::ONE * 0.2)
                .color(Color::BLUE)
                .anchor(Anchor::Bottom)
                .build(),
            Cube::default()
                .position(V3::Z)
                .size(V3::ONE * 0.2)
                .color(Color::CYAN)
                .anchor(Anchor::Bottom)
                .build(),
            Cube::default()
                .position(V3::XZ.normalize())
                .size(V3::ONE * 0.2)
                .color(Color::SKY)
                .anchor(Anchor::Bottom)
                .build(),
            Sphere::default()
                .color(Color::PURPLE)
                .material(Material::METAL)
                .anchor(Anchor::Bottom)
                .build()
        ])
        .run();
}

