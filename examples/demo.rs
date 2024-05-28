// dacho/examples/demo.rs

use dacho::prelude::*;

fn main() {
    let mut world = World::new();

    world
        .add(Cube(
            V3::new( 0.0, -0.2,  0.0), // position
            V3::new(10.0,  0.4, 10.0), // size
            Color::WHITE,              // base color
            V2::new(0.0, 0.9)          // metallic roughness
        ))
        .add(Sphere(
            V3::new(0.0, 0.5, 0.0), // position
            0.5,                    // radius
            Color::GREEN,           // base color
            V2::new(0.9, 0.7)       // metallic roughness
        ));

    run(&world);
}

