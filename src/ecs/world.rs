// dacho/src/ecs/world.rs

// std
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::FnOnce
};

// super
use super::{
    component::Component,
    entity::Entity
};

pub struct World {
    entities:          HashMap<u64, Entity>,
    components:        HashMap<u64, (TypeId, Box<dyn Any>)>,
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

    pub fn spawn_child_entity(&mut self, parent_id: u64) -> Option<u64> {
        let id = self.entity_counter;

        match self.get_mut_entity(parent_id) {
            Some(parent) => {
                parent.children_ids.push(id);
            },
            None => {
                return None;
            }
        }

        self.entity_counter += 1;

        self.entities.insert(id, Entity::new_child(id, parent_id));

        Some(id)
    }

    pub fn spawn_component<T: Component + 'static>(&mut self, entity_id: u64, component: T) {
        if self.get_entity(entity_id).is_none() {
            return;
        }

        let id = self.component_counter;
        self.component_counter += 1;

        self.components.insert(id, (TypeId::of::<T>(), Box::new(component)));

        if let Some(entity) = self.get_mut_entity(entity_id) {
            entity.components_ids.push(id);
        }
    }

    #[must_use]
    pub fn get_entity(&self, id: u64) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_mut_entity(&mut self, id: u64) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    #[must_use]
    pub fn get_component<T: Component + 'static>(&self, entity_id: u64) -> Option<&T> {
        let components_ids = {
            match self.get_entity(entity_id) {
                Some(entity) => entity.components_ids.clone(),
                None         => {
                    return None;
                }
            }
        };

        for component_id in &components_ids {
            if let Some(component) = self.components.get(component_id) {
                if component.0 == TypeId::of::<T>() {
                    return component.1.downcast_ref::<T>();
                }
            }
        }

        None
    }

    pub fn get_mut_component<T: Component + 'static, F>(&mut self, entity_id: u64, closure: F)
    where
        F: FnOnce(Option<&mut T>)
    {
        let components_ids = {
            if let Some(entity) = self.get_entity(entity_id) {
                entity.components_ids.clone()
            } else {
                closure(None);

                return;
            }
        };

        let mut id = u64::MAX;

        for component_id in &components_ids {
            if let Some((component_type, _)) = self.components.get(component_id) {
                if *component_type == TypeId::of::<T>() {
                    id = *component_id;

                    break;
                }
            }
        }

        closure(
            match id {
                u64::MAX => None,
                _        => {
                    match self.components.get_mut(&id) {
                        Some((_, component)) => component.downcast_mut::<T>(),
                        None                 => None
                    }
                }
            }
        );
    }

    pub fn remove_entity(&mut self, id: u64) {
        let parent_id_option = {
            match self.get_entity(id) {
                Some(entity) => entity.parent_id_option,
                None         => {
                    return;
                }
            }
        };

        if let Some(parent_id) = parent_id_option {
            if let Some(parent) = self.get_mut_entity(parent_id) {
                for i in 0..parent.children_ids.len() {
                    if parent.children_ids[i] == id {
                        parent.children_ids.remove(i);

                        break;
                    }
                }
            }
        }

        self.remove_entity_(id);
    }

    // recursive functionality of Self::remove_entity
    fn remove_entity_(&mut self, id: u64) {
        let (children_ids, components_ids) = {
            match self.get_entity(id) {
                Some(entity) => (entity.children_ids.clone(), entity.components_ids.clone()),
                None         => {
                    return;
                }
            }
        };

        for child_id in &children_ids {
            self.remove_entity_(*child_id);
        }

        for component_id in &components_ids {
            self.components.remove(component_id);
        }

        self.entities.remove(&id);
    }

    pub fn remove_component<T: Component + 'static>(&mut self, entity_id: u64) {
        self.remove_component_::<T>(entity_id, false);
    }

    pub fn remove_components<T: Component + 'static>(&mut self, entity_id: u64) {
        self.remove_component_::<T>(entity_id, true);
    }

    // optionally recursive functionality of Self::remove_component(s)
    fn remove_component_<T: Component + 'static>(&mut self, entity_id: u64, recursive: bool) {
        let components_ids = {
            match self.get_entity(entity_id) {
                Some(entity) => entity.components_ids.clone(),
                None         => {
                    return;
                }
            }
        };

        for component_id in &components_ids {
            if let Some(component) = self.components.get(component_id) {
                if component.0 == TypeId::of::<T>() {
                    if let Some(entity) = self.get_mut_entity(entity_id) {
                        for i in 0..entity.components_ids.len() {
                            if entity.components_ids[i] == *component_id {
                                entity.components_ids.remove(i);
                                break;
                            }
                        }

                        self.components.remove(component_id);

                        if !recursive {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn call(&self, callback: fn(&Self, &[u64]), ids: &[u64]) {
        callback(self, ids);
    }

    pub fn call_mut(&mut self, callback: fn(&mut Self, &[u64], &dyn Any), ids: &[u64], data: &dyn Any) {
        callback(self, ids, data);
    }

    pub fn debug(&self) {
        dbg!(&self.entities);

        print!("&self.components = {{ ");

        for (k, v) in &self.components {
            print!("{k}, ");
        }

        println!("}}");
    }
}

