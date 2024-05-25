// dacho/src/prelude/shapes.rs

use super::types::{V2, V3};

pub enum Object {
    //      position size color metrou
    Cube   (V3,      V3,  V3,   V2),
    Sphere (V3,      f32, V3,   V2)
}

