// dacho/src/prelude/object/shape2D.rs

// crates
use serde::{Serialize, Deserialize};

// crate
use crate::prelude::types::{V2, V3};

#[derive(Clone, Serialize, Deserialize)]
pub enum InnerShape2D {
    Quad   { position: V3, size:   V2                 },
    Circle { position: V3, radius: f32, points: usize }
}

