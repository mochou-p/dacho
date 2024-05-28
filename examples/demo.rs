// dacho/examples/demo.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    world
        .add(Cube(
            V3::new( 0.0, -0.2,  0.0), // position
            V3::new(10.0,  0.4, 10.0), // size
            Color::DARK_BLUE,          // base color
            Material::ROUGH            // metallic roughness
        ))
        .add(Sphere(
            V3::new(0.0, 0.5, 0.0), // position
            0.5,                    // radius
            Color::CYAN,            // base color
            Material::METAL         // metallic roughness
        ));

    run(&world);
}

