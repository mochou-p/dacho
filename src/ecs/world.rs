// dacho/src/ecs/world.rs

// std
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ops::Fn
};

// super
use super::{
    component::Component,
    entity::Entity
};

pub type Id = u32;

pub struct World {
    entities:          HashMap<Id, Entity>,
    components:        HashMap<Id, (TypeId, Box<dyn Any>)>,
    entity_counter:    Id,
    component_counter: Id
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

    pub fn spawn_entity(&mut self) -> Id {
        let id = self.entity_counter;
        self.entity_counter += 1;

        self.entities.insert(id, Entity::new(id));

        id
    }

    pub fn spawn_child_entity(&mut self, parent_id: Id) -> Option<Id> {
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

    pub fn spawn_component<T: Component + 'static>(&mut self, entity_id: Id, component: T) {
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

    #[inline]
    #[must_use]
    pub fn get_entity(&self, id: Id) -> Option<&Entity> {
        self.entities.get(&id)
    }

    #[inline]
    pub fn get_mut_entity(&mut self, id: Id) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    #[must_use]
    pub fn get_component<T: Component + 'static>(&self, entity_id: Id) -> Option<&T> {
        let components_ids = {
            match self.get_entity(entity_id) {
                Some(entity) => entity.components_ids.clone(),
                None         => {
                    return None;
                }
            }
        };

        let user_type = TypeId::of::<T>();

        for component_id in &components_ids {
            if let Some(component) = self.components.get(component_id) {
                if component.0 == user_type {
                    return component.1.downcast_ref::<T>();
                }
            }
        }

        None
    }

    #[must_use]
    pub fn get_components<T: Component + 'static>(&self, entity_id: Id) -> Vec<&T> {
        let components_ids = {
            match self.get_entity(entity_id) {
                Some(entity) => entity.components_ids.clone(),
                None         => {
                    return vec![];
                }
            }
        };

        let     user_type  = TypeId::of::<T>();
        let mut components = vec![];

        for component_id in &components_ids {
            if let Some(component) = self.components.get(component_id) {
                if component.0 == user_type {
                    if let Some(downcasted_component) = component.1.downcast_ref::<T>() {
                        components.push(downcasted_component);
                    }
                }
            }
        }

        components
    }

    #[inline]
    pub fn get_mut_component<T: Component + 'static, F>(&mut self, entity_id: Id, closure: F)
    where
        F: Fn(&mut T)
    {
        self.get_mut_component_(entity_id, closure, false);
    }

    #[inline]
    pub fn get_mut_components<T: Component + 'static, F>(&mut self, entity_id: Id, closure: F)
    where
        F: Fn(&mut T)
    {
        self.get_mut_component_(entity_id, closure, true);
    }

    // optionally recursive functionality of Self::get_mut_component(s)
    fn get_mut_component_<T: Component + 'static, F>(&mut self, entity_id: Id, closure: F, recursive: bool)
    where
        F: Fn(&mut T)
    {
        let components_ids = {
            if let Some(entity) = self.get_entity(entity_id) {
                entity.components_ids.clone()
            } else {
                return;
            }
        };

        let user_type = TypeId::of::<T>();

        for component_id in &components_ids {
            if let Some((component_type, _)) = self.components.get(component_id) {
                if *component_type == user_type {
                    if let Some((_, component)) = self.components.get_mut(component_id) {
                        if let Some(downcasted_component) = component.downcast_mut::<T>() {
                            closure(downcasted_component);
                        }
                    }

                    if !recursive {
                        break;
                    }
                }
            }
        }
    }

    pub fn remove_entity(&mut self, id: Id) {
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
    fn remove_entity_(&mut self, id: Id) {
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

    #[inline]
    pub fn remove_component<T: Component + 'static>(&mut self, entity_id: Id) {
        self.remove_component_::<T>(entity_id, false);
    }

    #[inline]
    pub fn remove_components<T: Component + 'static>(&mut self, entity_id: Id) {
        self.remove_component_::<T>(entity_id, true);
    }

    // optionally recursive functionality of Self::remove_component(s)
    fn remove_component_<T: Component + 'static>(&mut self, entity_id: Id, recursive: bool) {
        let components_ids = {
            match self.get_entity(entity_id) {
                Some(entity) => entity.components_ids.clone(),
                None         => {
                    return;
                }
            }
        };

        let user_type = TypeId::of::<T>();

        for component_id in &components_ids {
            if let Some(component) = self.components.get(component_id) {
                if component.0 == user_type {
                    // this World::get_mut_entity call is reduntantly inside a for loop,
                    // but it is here to satisfy the borrow checker
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

    #[inline]
    pub fn call(&self, callback: fn(&Self, &[Id]), ids: &[Id]) {
        callback(self, ids);
    }

    #[inline]
    pub fn call_mut<T>(&mut self, callback: fn(&mut Self, &[Id], T), ids: &[Id], data: T) {
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

