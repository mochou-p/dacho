// dacho/src/prelude/materials.rs

use super::types::V2;

pub struct Material;

impl Material {
    pub const ROUGH: V2 = V2::new(0.0, 1.0);
    pub const METAL: V2 = V2::new(0.9, 0.1);
}

