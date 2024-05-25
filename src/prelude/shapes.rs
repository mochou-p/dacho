// dacho/src/prelude/shapes.rs

use super::types::V3;

#[derive(Debug)]
pub enum Object {
    Cube   (V3, V3),
    Sphere (V3, f32)
}

