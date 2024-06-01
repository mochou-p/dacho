// dacho/examples/primitives.rs

use dacho::prelude::*;

// See the Object enum in dacho::prelude::shapes
// for all currently available primitives

fn main() {
    World::new()
        .add(&[
            // ::default() creates a unit-sized primitive
            // with sensible defaults for all properties,
            // usually chained with other functions (setters)
            // to overwrite specific properties
            Cube::default()

                // .size() sets the size of the primitive,
                // argument type varies with primitive type
                .size(V3::new(100.0, 0.02, 100.0))

                // .anchor() sets the origin point,
                //
                // setting Anchor::Top while y=0,
                // the top of the primitive will now be at y=0
                // e.g. useful for ground, platforms, etc.
                .anchor(Anchor::Top)

                // .build() converts primitives to a shared type
                // to allow storing them in one non-dyn collection
                .build(),



            Sphere::default()

                // .position() sets the position of the center,
                // currently preferably called before .anchor()
                .position(V3::X * 2)

                // setting Anchor::Bottom while y=0,
                // the bottom of the primitive will now be at y=0
                // e.g. useful for grounded things like characters,
                // to keep the feet on the ground
                .anchor(Anchor::Bottom)

                // .color() sets the color
                // use predefined `Color`s or a custom V3 as RGB
                .color(Color::BLUE)

                // .material() sets the metalness and roughness (PBR)
                // use predefined `Material`s or a custom V2
                .material(Material::METAL)

                .build(),



            // ::new() takes in all properties as arguments,
            // if you prefer a more compact and specific declaration
            Cube::new(V3::X * -0.420, V3::ONE * 0.1337, Anchor::Bottom, Color::RED, Material::default())
                .build(),



            // you can also define these as basic structs,
            // (except for anchor due to current implementation)
            Cube {
                position: V3::XY  * 0.1337 * 0.5,
                size:     V3::ONE * 0.1337,
                color:    Color::GREEN,
                ..Default::default()
            }.build()
        ])
        .run();
}

