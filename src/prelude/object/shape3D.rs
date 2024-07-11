// dacho/src/prelude/object/shape3D.rs

// crates
use serde::{Serialize, Deserialize};

// crate
use crate::prelude::types::V3;

#[derive(Clone, Serialize, Deserialize)]
pub enum InnerShape3D {
    Cube   { position: V3, size:   V3                                 },
    Sphere { position: V3, radius: f32, sectors: usize, stacks: usize }
}

