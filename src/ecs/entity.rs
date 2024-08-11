// dacho/src/ecs/entity.rs

// core
use core::any::TypeId;

// std
use std::collections::HashMap;

// super
use super::world::Id;

#[non_exhaustive]
#[derive(Debug)]
pub struct Entity {
    pub id:                Id,
    pub parent_id_option:  Option<Id>,
    pub children_ids:      Vec<Id>,
    pub components_id_map: HashMap<TypeId, Vec<Id>>
}

impl Entity {
    #[must_use]
    pub fn new(id: Id) -> Self {
        Self {
            id,
            parent_id_option:  None,
            children_ids:      vec![],
            components_id_map: HashMap::new()
        }
    }

    #[must_use]
    pub fn new_child(id: Id, parent_id: Id) -> Self {
        Self {
            id,
            parent_id_option:  Some(parent_id),
            children_ids:      vec![],
            components_id_map: HashMap::new()
        }
    }
}

