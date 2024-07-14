// dacho/src/ecs/world.rs

// std
use std::collections::HashMap;

// super
use super::{
    component::Component,
    entity::Entity
};

pub struct World {
    entities:          HashMap<u64, Entity>,
    components:        HashMap<u64, Box<dyn Component>>,
    entity_counter:    u64,
    component_counter: u64
}

impl World {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            entities:          HashMap::new(),
            components:        HashMap::new(),
            entity_counter:    0,
            component_counter: 0
        }
    }

    pub fn spawn_entity(&mut self) -> u64 {
        let id = self.entity_counter;
        self.entity_counter += 1;

        self.entities.insert(id, Entity::new(id));

        id
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn spawn_child_entity(&mut self, parent_id: u64) -> u64 {
        let id = self.entity_counter;
        self.entity_counter += 1;

        {
            let mut parent = self.get_mut_entity(parent_id).expect("unexpected HashMap error");
            parent.children_ids.push(id);
        }

        self.entities.insert(id, Entity::new(id));

        let mut child = self.get_mut_entity(id).expect("unexpected HashMap error");
        child.parent  = Some(parent_id);

        id
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn spawn_component<T: Component + 'static>(&mut self, entity_id: u64, component: T) {
        let id = self.component_counter;
        self.component_counter += 1;

        self.components.insert(id, Box::new(component));

        let mut entity = self.get_mut_entity(entity_id).expect("unexpected HashMap error");
        entity.component_ids.push(id);
    }

    #[must_use]
    pub fn get_entity(&self, id: u64) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_mut_entity(&mut self, id: u64) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn debug(&self) {
        dbg!(&self.entities);
    }
}

