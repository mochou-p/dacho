// dacho/src/ecs/entity.rs

// super
use super::{
    component::Component,
    world::Id
};

#[derive(Debug)]
pub struct Entity {
    pub id:               Id,
    pub parent_id_option: Option<Id>,
    pub children_ids:     Vec<Id>,
    pub components_ids:   Vec<Id>
}

impl Entity {
    #[must_use]
    pub fn new(id: Id) -> Self {
        Self { id, parent_id_option: None, children_ids: vec![], components_ids: vec![] }
    }

    #[must_use]
    pub fn new_child(id: Id, parent_id: Id) -> Self {
        Self { id, parent_id_option: Some(parent_id), children_ids: vec![], components_ids: vec![] }
    }
}

