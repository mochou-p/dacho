// dacho/src/prelude/materials.rs

use super::types::V2;

pub struct Material;

impl Material {
    pub const ROUGH: V2 = V2::new(0.05, 0.95);
    pub const METAL: V2 = V2::new(0.88, 0.11);
}

