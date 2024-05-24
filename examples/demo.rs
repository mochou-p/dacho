// dacho/examples/demo.rs

use dacho::prelude::*; 

fn main() {
    let mut scene = Scene::new();

    scene
        .add(Cube(
            V3::new(0.0, -0.5, 0.0),
            V3::new(1.0,  1.0, 1.0)
        ))
        .add(Sphere(
            V3::new(0.0, 0.5, 0.0),
            0.5
        ));

    run(&scene);
}

