// dacho/core/ecs/src/entity.rs

use {
    core::any::{Any, TypeId, type_name},
    std::collections::HashMap
};

use super::query::QueryT;


pub struct Entity {
    pub(crate) components: HashMap<TypeId, Vec<Box<dyn Any>>>
}

impl Entity {
    pub(crate) fn new<T: QueryT + 'static>(tuple: T) -> Self {
        Self { components: tuple.to_components() }
    }

    #[must_use]
    pub fn has<T>(&self) -> bool
    where
        T: 'static
    {
        self.components
            .contains_key(&TypeId::of::<T>())
    }

    #[must_use]
    pub fn count<T>(&self) -> usize
    where
        T: 'static
    {
        if let Some(components) = self.components.get(&TypeId::of::<T>()) {
            return components.len();
        }

        0
    }

    #[must_use]
    #[expect(clippy::unwrap_used,      reason = "guarded by how the `HashMap` stores components")]
    pub fn first<T>(&self) -> Option<&T>
    where
        T: 'static
    {
        if let Some(components) = self.components.get(&TypeId::of::<T>()) {
            return Some(components[0].downcast_ref::<T>().unwrap());
        }

        None
    }

    #[must_use]
    #[expect(clippy::panic,       reason = "the function is named `*_unchecked`")]
    #[expect(clippy::unwrap_used, reason = "guarded by how the `HashMap` stores components")]
    pub fn first_unchecked<T>(&self) -> &T
    where
        T: 'static
    {
        self.components
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("there is no `{}`", type_name::<T>()))
            [0]
            .downcast_ref::<T>()
            .unwrap()
    }

    #[must_use]
    #[expect(clippy::unwrap_used,      reason = "guarded by how the `HashMap` stores components")]
    pub fn first_mut<T>(&mut self) -> Option<&mut T>
    where
        T: 'static
    {
        if let Some(components) = self.components.get_mut(&TypeId::of::<T>()) {
            return Some(components[0].downcast_mut::<T>().unwrap());
        }

        None
    }

    #[must_use]
    #[expect(clippy::panic,       reason = "the function is named `*_unchecked`")]
    #[expect(clippy::unwrap_used, reason = "guarded by how the `HashMap` stores components")]
    pub fn first_mut_unchecked<T>(&mut self) -> &mut T
    where
        T: 'static
    {
        self.components
            .get_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("there is no `{}`", type_name::<T>()))
            [0]
            .downcast_mut::<T>()
            .unwrap()
    }

    #[must_use]
    #[expect(clippy::unwrap_used,      reason = "guarded by how the `HashMap` stores components")]
    pub fn iter<T>(&self) -> Option<impl Iterator<Item = &T>>
    where
        T: 'static
    {
        if let Some(components) = self.components.get(&TypeId::of::<T>()) {
            return Some(
                components.iter()
                    .map(|component| component.downcast_ref::<T>().unwrap())
            );
        }

        None
    }

    #[expect(clippy::panic,       reason = "the function is named `*_unchecked`")]
    #[expect(clippy::unwrap_used, reason = "guarded by how the `HashMap` stores components")]
    pub fn iter_unchecked<T>(&self) -> impl Iterator<Item = &T>
    where
        T: 'static
    {
        self.components
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("there are no `{}`s", type_name::<T>()))
            .iter()
            .map(|component| component.downcast_ref::<T>().unwrap())
    }

    #[must_use]
    #[expect(clippy::unwrap_used,      reason = "guarded by how the `HashMap` stores components")]
    pub fn iter_mut<T>(&mut self) -> Option<impl Iterator<Item = &mut T>>
    where
        T: 'static
    {
        if let Some(components) = self.components.get_mut(&TypeId::of::<T>()) {
            return Some(
                components.iter_mut()
                    .map(|component| component.downcast_mut::<T>().unwrap())
            );
        }

        None
    }

    #[expect(clippy::panic,       reason = "the function is named `*_unchecked`")]
    #[expect(clippy::unwrap_used, reason = "guarded by how the `HashMap` stores components")]
    pub fn iter_mut_unchecked<T>(&mut self) -> impl Iterator<Item = &mut T>
    where
        T: 'static
    {
        self.components
            .get_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("there are no `{}`s", type_name::<T>()))
            .iter_mut()
            .map(|component| component.downcast_mut::<T>().unwrap())
    }
}

