// dacho/src/ecs/world.rs

// std
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    mem::take
};

// crates
use {
    anyhow::Result,
    winit::{
        dpi::PhysicalPosition,
        event::{
            ElementState, KeyEvent, MouseButton,
            MouseScrollDelta::{self, LineDelta},
        },
        keyboard::{KeyCode, PhysicalKey::Code},
    }
};

// super
use super::{
    component::Component,
    entity::Entity,
    system::Systems
};

// crate
use crate::{
    prelude::mesh::Mesh,
    renderer::rendering::GeometryData
};

pub type Id    = u32;
pub type State = u8;

pub struct World {
    entities:           HashMap<Id, Entity>,
    components:         HashMap<Id, Box<dyn Any>>,
    entity_counter:     Id,
    component_counter:  Id,
    mesh_components:    Vec<Id>,

    pub(crate) systems: Systems,
}

impl World {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub(crate) fn new() -> Self {
        Self {
            entities:          HashMap::new(),
            components:        HashMap::new(),
            entity_counter:    0,
            component_counter: 0,
            systems:           Systems::new(),
            mesh_components:   vec![]
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

    pub fn spawn_component<T>(&mut self, entity_id: Id, component: T)
    where
        T: Component
    {
        if self.get_entity(entity_id).is_none() {
            return;
        }

        let id = self.component_counter;
        self.component_counter += 1;

        self.components.insert(id, Box::new(component));

        let user_type = TypeId::of::<T>();

        if user_type == TypeId::of::<Mesh>() {
            self.mesh_components.push(id);
        }

        if let Some(entity) = self.get_mut_entity(entity_id) {
            entity.components_id_map
                .entry(user_type)
                .or_insert_with(|| Vec::with_capacity(1))
                .push(id);
        }
    }

    pub fn spawn_components<T>(&mut self, entity_id: Id, amount: u32, component: T)
    where
        T: Component + Copy
    {
        if self.get_entity(entity_id).is_none() {
            return;
        }

        let range = self.component_counter..self.component_counter + amount;

        range.clone()
            .map(|i| self.components.insert(i, Box::new(component)));

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

    pub fn get_entity_component<T>(&self, entity_id: Id, closure: impl FnOnce(&T))
    where
        T: Component
    {
        if let Some(entity) = self.get_entity(entity_id) {
            if let Some(components_ids) = entity.components_id_map.get(&TypeId::of::<T>()) {
                if let Some(component_id) = components_ids.first() {
                    if let Some(component) = self.components.get(component_id) {
                        if let Some(downcasted_component) = component.downcast_ref::<T>() {
                            closure(downcasted_component);
                        }
                    }
                }
            }
        }
    }

    pub fn get_entity_components<T>(&self, entity_id: Id, closure: impl Fn(&T))
    where
        T: Component
    {
        if let Some(entity) = self.get_entity(entity_id) {
            if let Some(components_ids) = entity.components_id_map.get(&TypeId::of::<T>()) {
                for component_id in components_ids {
                    if let Some(component) = self.components.get(component_id) {
                        if let Some(downcasted_component) = component.downcast_ref::<T>() {
                            closure(downcasted_component);
                        }
                    }
                }
            }
        }
    }

    pub fn get_entity_mut_component<T>(&mut self, entity_id: Id, closure: impl FnOnce(&mut T))
    where
        T: Component
    {
        if let Some(entity) = self.get_entity(entity_id) {
            if let Some(components_ids) = entity.components_id_map.get(&TypeId::of::<T>()) {
                if let Some(component_id) = components_ids.first() {
                    if let Some(component) = self.components.get_mut(&component_id.clone()) {
                        if let Some(downcasted_component) = component.downcast_mut::<T>() {
                            closure(downcasted_component);
                        }
                    }
                }
            }
        }
    }

    pub fn get_entity_mut_components<T>(&mut self, entity_id: Id, closure: impl Fn(&mut T))
    where
        T: Component
    {
        if let Some(entity) = self.get_entity(entity_id) {
            if let Some(components_ids) = entity.components_id_map.get(&TypeId::of::<T>()) {
                for component_id in components_ids.clone() {
                    if let Some(component) = self.components.get_mut(&component_id) {
                        if let Some(downcasted_component) = component.downcast_mut::<T>() {
                            closure(downcasted_component);
                        }
                    }
                }
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
    pub fn remove_component<T>(&mut self, entity_id: Id)
    where
        T: Component
    {
        self.remove_component_::<T>(entity_id, false);
    }

    #[inline]
    pub fn remove_components<T>(&mut self, entity_id: Id)
    where
        T: Component
    {
        self.remove_component_::<T>(entity_id, true);
    }

    fn remove_component_<T>(&mut self, entity_id: Id, recursive: bool)
    where
        T: Component
    {
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

    pub fn get_state(&self, closure: impl FnOnce(State)) {
        if let Some((state, _)) = self.systems.state {
            closure(state);
        }
    }

    pub fn set_state(&mut self, new_state: State) {
        if self.systems.state.is_none() {
            return;
        }

        let taken_state_system = take(&mut self.systems.state);

        if let Some((mut old_state, state_system)) = &taken_state_system {
            state_system(self, old_state, new_state);

            old_state = new_state;
        }

        self.systems.state = taken_state_system;
    }

    pub(crate) fn start(&mut self) {
        if self.systems.start.is_empty() {
            return;
        }

        let taken_start_systems = take(&mut self.systems.start);

        for start_system in taken_start_systems {
            start_system(self);
        }
    }

    pub(crate) fn update(&mut self) {
        if self.systems.update.is_empty() {
            return;
        }

        let taken_update_systems = take(&mut self.systems.update);

        for update_system in &taken_update_systems {
            update_system(self);
        }

        self.systems.update = taken_update_systems;
    }

    pub(crate) fn keyboard(&mut self, key_event: &KeyEvent) {
        if self.systems.keyboard.is_empty() {
            return;
        }

        if let Code(code) = key_event.physical_key {
            let taken_keyboard_systems = take(&mut self.systems.keyboard);

            for keyboard_system in &taken_keyboard_systems {
                keyboard_system(self, code, key_event.state);
            }

            self.systems.keyboard = taken_keyboard_systems;
        }
    }

    pub(crate) fn mouse_position(&mut self, position: PhysicalPosition<f64>) {
        if self.systems.mouse_position.is_empty() {
            return;
        }

        let taken_mouse_position_systems = take(&mut self.systems.mouse_position);

        for mouse_position_system in &taken_mouse_position_systems {
            mouse_position_system(self, position);
        }

        self.systems.mouse_position = taken_mouse_position_systems;
    }

    pub(crate) fn mouse_buttons(&mut self, button: MouseButton, action: ElementState) {
        if self.systems.mouse_button.is_empty() {
            return;
        }

        let taken_mouse_button_systems = take(&mut self.systems.mouse_button);

        for mouse_button_system in &taken_mouse_button_systems {
            mouse_button_system(self, button, action);
        }

        self.systems.mouse_button = taken_mouse_button_systems;
    }

    pub(crate) fn mouse_wheel(&mut self, delta: MouseScrollDelta) {
        if self.systems.mouse_wheel.is_empty() {
            return;
        }

        if let LineDelta(x, y) = delta {
            let taken_mouse_wheel_systems = take(&mut self.systems.mouse_wheel);

            for mouse_wheel_system in &taken_mouse_wheel_systems {
                mouse_wheel_system(self, x, y);
            }

            self.systems.mouse_wheel = taken_mouse_wheel_systems;
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub(crate) fn get_mesh_data(&mut self) -> Result<Vec<GeometryData>> {
        let mut mesh_data = vec![];

        for component_id in &self.mesh_components {
            if let Some(component) = self.components.get(component_id) {
                if let Some(mesh) = component.downcast_ref::<Mesh>() {
                    mesh_data.push((mesh.data_builder)()?);
                }
            }
        }

        Ok(mesh_data)
    }
}

