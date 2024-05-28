// dacho/examples/demo.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    world
        .add(Cube(
            V3::new(0.0, -0.2, 0.0), // position
            V3::new(5.0,  0.4, 5.0), // size
            Color::DARK,             // base color
            Material::ROUGH          // metallic roughness
        ))
        .add(Sphere(
            V3::new(0.0, 0.5, 0.0), // position
            0.5,                    // radius
            Color::BLUE,            // base color
            Material::METAL         // metallic roughness
        ));

    run(&world);
}

