// dacho/src/prelude/materials.rs

// super
use super::types::V2;

pub struct Material;

impl Material {
    pub const ROUGH: V2 = V2::new(0.05, 0.95);
    pub const MIXED: V2 = V2::new(0.40, 0.60);
    pub const METAL: V2 = V2::new(0.88, 0.11);

    #[allow(clippy::should_implement_trait)]
    pub fn default() -> V2 {
        Self::MIXED
    }
}
