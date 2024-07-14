// dacho/src/ecs/entity.rs

// super
use super::component::Component;

#[derive(Debug)]
pub struct Entity {
    pub id:             u64,
    pub parent:         Option<u64>,
    pub children_ids:   Vec<u64>,
    pub components_ids: Vec<u64>
}

impl Entity {
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self { id, parent: None, children_ids: vec![], components_ids: vec![] }
    }
}

