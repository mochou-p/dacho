// dacho/src/ecs/world.rs

// std
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    mem::take,
    ops::{Fn, FnOnce}
};

// super
use super::{
    component::Component,
    entity::Entity
};

pub type Id             = u32;
    type BoxedDynFnOnce = Box<dyn FnOnce(&mut World)>;
    type BoxedDynFn     = Box<dyn Fn    (&mut World)>;

pub struct World {
    entities:          HashMap<Id, Entity>,
    components:        HashMap<Id, Box<dyn Any>>,
    entity_counter:    Id,
    component_counter: Id,
    start_callbacks:   Vec<BoxedDynFnOnce>,
    update_callbacks:  Vec<BoxedDynFn>
}

impl World {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            entities:          HashMap::new(),
            components:        HashMap::new(),
            entity_counter:    0,
            component_counter: 0,
            start_callbacks:   vec![],
            update_callbacks:  vec![]
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
            _ => {
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

        self.components.insert(id, Box::new(component));

        if let Some(entity) = self.get_mut_entity(entity_id) {
            entity.components_id_map
                .entry(TypeId::of::<T>())
                .or_insert_with(|| Vec::with_capacity(1))
                .push(id);
        }
    }

    pub fn spawn_components<T: Component + Copy + 'static>(&mut self, entity_id: Id, amount: u32, component: T) {
        if self.get_entity(entity_id).is_none() {
            return;
        }

        let range = self.component_counter..self.component_counter + amount;

        for i in range.clone() {
            self.components.insert(i, Box::new(component));
        }

        if let Some(entity) = self.get_mut_entity(entity_id) {
            let capacity = amount as usize;

            entity.components_id_map
                .entry(TypeId::of::<T>())
                .and_modify(|components_ids| {
                    components_ids.reserve_exact(capacity);
                })
                .or_insert_with(|| Vec::with_capacity(capacity))
                .extend(range);
        }

        self.component_counter += amount;
    }

    #[inline]
    #[must_use]
    fn get_entity(&self, id: Id) -> Option<&Entity> {
        self.entities.get(&id)
    }

    #[inline]
    #[must_use]
    fn get_mut_entity(&mut self, id: Id) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    #[must_use]
    pub fn get_component<T: Component + 'static>(&self, entity_id: Id) -> Option<&T> {
        if let Some(entity) = self.get_entity(entity_id) {
            if let Some(components_ids) = entity.components_id_map.get(&TypeId::of::<T>()) {
                if let Some(component) = self.components.get(&components_ids[0]) {
                    return component.downcast_ref::<T>();
                }
            }
        }

        None
    }

    #[must_use]
    pub fn get_components<T: Component + 'static>(&self, entity_id: Id) -> Vec<&T> {
        if let Some(entity) = self.get_entity(entity_id) {
            if let Some(components_ids) = entity.components_id_map.get(&TypeId::of::<T>()) {
                let mut components = Vec::with_capacity(components_ids.len());

                for component_id in components_ids {
                    if let Some(component) = self.components.get(component_id) {
                        if let Some(downcasted_component) = component.downcast_ref::<T>() {
                            components.push(downcasted_component);
                        }
                    }
                }

                return components;
            }
        }

        vec![]
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
            match self.get_entity(entity_id) {
                Some(entity) => {
                    match entity.components_id_map.get(&TypeId::of::<T>()) {
                        Some(components_ids) => components_ids.clone(),
                        _                    => {
                            return;
                        }
                    }
                },
                _ => {
                    return;
                }
            }
        };

        for component_id in &components_ids {
            if let Some(component) = self.components.get_mut(component_id) {
                if let Some(downcasted_component) = component.downcast_mut::<T>() {
                    closure(downcasted_component);
                }
            }

            if !recursive {
                break;
            }
        }
    }

    pub fn remove_entity(&mut self, id: Id) {
        let parent_id_option = {
            match self.get_entity(id) {
                Some(entity) => entity.parent_id_option,
                _            => {
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
        let (children_ids, components_id_map) = {
            match self.get_entity(id) {
                Some(entity) => (entity.children_ids.clone(), entity.components_id_map.clone()),
                _            => {
                    return;
                }
            }
        };

        for child_id in &children_ids {
            self.remove_entity_(*child_id);
        }

        for components_ids in components_id_map.values() {
            for component_id in components_ids {
                self.components.remove(component_id);
            }
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
        let user_type = &TypeId::of::<T>();

        let components_ids = {
            match self.get_entity(entity_id) {
                Some(entity) => {
                    match entity.components_id_map.get(user_type) {
                        Some(components_ids) => components_ids.clone(),
                        _ => {
                            return;
                        }
                    }
                },
                _ => {
                    return;
                }
            }
        };

        for component_id in &components_ids {
            self.components.remove(component_id);

            if !recursive {
                break;
            }
        }

        if let Some(entity) = self.get_mut_entity(entity_id) {
            entity.components_id_map.remove(user_type);
        }
    }

    pub fn start(&mut self, callback: impl FnOnce(&mut Self) + 'static) {
        self.start_callbacks.push(Box::new(callback));
    }

    pub fn update(&mut self, callback: impl Fn(&mut Self) + 'static) {
        self.update_callbacks.push(Box::new(callback));
    }

    pub fn run(&mut self) {
        {
            let mut taken_start_callbacks = take(&mut self.start_callbacks);

            for callback in taken_start_callbacks {
                callback(self);
            }
        }

        let taken_update_callbacks = take(&mut self.update_callbacks);

        loop {
            for callback in &taken_update_callbacks {
                callback(self);
            }
        }
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

