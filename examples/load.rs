// dacho/examples/load.rs

use dacho::prelude::*;

fn main() {
    World::load("saved_demo")
        .run();
}

