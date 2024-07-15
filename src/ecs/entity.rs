// dacho/src/ecs/entity.rs

// super
use super::component::Component;

#[derive(Debug)]
pub struct Entity {
    pub id:               u64,
    pub parent_id_option: Option<u64>,
    pub children_ids:     Vec<u64>,
    pub components_ids:   Vec<u64>
}

impl Entity {
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self { id, parent_id_option: None, children_ids: vec![], components_ids: vec![] }
    }

    #[must_use]
    pub fn new_child(id: u64, parent_id: u64) -> Self {
        Self { id, parent_id_option: Some(parent_id), children_ids: vec![], components_ids: vec![] }
    }
}

