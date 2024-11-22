// dacho/core/ecs/src/entity.rs

use {
    core::any::{Any, TypeId, type_name},
    alloc::collections::BTreeSet,
    std::collections::HashMap
};

use super::{query::QueryT, world::WorldComponent};


pub struct Entity {
    pub(crate) components: HashMap<TypeId, Vec<Box<dyn Any>>>
}

#[expect(clippy::panic,       reason = "`*_unchecked` functions")]
#[expect(clippy::unwrap_used, reason = "safe, guarded by context")]
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

    pub fn insert<T: 'static>(&mut self, component: T, world: &WorldComponent) {
        let ti  = TypeId::of::<T>();
        let ptr = self as *const _;
        let set = self.components.keys().cloned().collect();

        self.components
            .entry(ti)
            .or_insert_with(|| {
                world.change(set, ptr, &ti, true);

                Vec::with_capacity(1)
            })
            .push(Box::new(component));
    }

    pub fn insert_n<T: Copy + 'static, const N: usize>(&mut self, component: T, world: &WorldComponent) {
        let ti  = TypeId::of::<T>();
        let ptr = self as *const _;
        let set = self.components.keys().cloned().collect();

        self.components
            .entry(ti)
            .and_modify(|vec| vec.extend([component; N].map(|x| Box::new(x) as Box<dyn Any>)))
            .or_insert_with(|| {
                world.change(set, ptr, &ti, true);

                vec![component; N].into_iter().map(|x| Box::new(x) as Box<dyn Any>).collect()
            });
    }

    #[inline]
    pub fn remove_first<T: 'static>(&mut self, world: &WorldComponent) -> Option<T> {
        self.remove_nth::<T>(0, world)
    }

    #[inline]
    pub fn remove_first_unchecked<T: 'static>(&mut self, world: &WorldComponent) -> T {
        self.remove_nth_unchecked::<T>(0, world)
    }

    pub fn remove_nth<T: 'static>(&mut self, index: usize, world: &WorldComponent) -> Option<T> {
        let ti  = TypeId::of::<T>();

        if let Some(vec) = self.components.get_mut(&ti) {
            if vec.len() > index {
                let component = *vec.swap_remove(index).downcast::<T>().unwrap();

                if vec.is_empty() {
                    world.change(
                        self.components.keys().cloned().collect(),
                        self as *const _,
                        &ti,
                        false
                    );
                }

                return Some(component);
            }
        }

        None
    }

    pub fn remove_nth_unchecked<T: 'static>(&mut self, index: usize, world: &WorldComponent) -> T {
        let ti  = TypeId::of::<T>();

        let vec       = self.components.get_mut(&ti).unwrap();
        let component = *vec.swap_remove(index).downcast::<T>().unwrap();

        if vec.is_empty() {
            world.change(
                self.components.keys().cloned().collect(),
                self as *const _,
                &ti,
                false
            );
        }

        component
    }

    #[inline]
    pub fn remove_last<T: 'static>(&mut self, world: &WorldComponent) -> Option<T> {
        if let Some(vec) = self.components.get(&TypeId::of::<T>()) {
            return self.remove_nth::<T>(vec.len() - 1, world);
        }

        None
    }

    #[inline]
    pub fn remove_last_unchecked<T: 'static>(&mut self, world: &WorldComponent) -> T {
        self.remove_nth_unchecked::<T>(
            self.components[&TypeId::of::<T>()].len() - 1,
            world
        )
    }

    pub fn remove_all<T: 'static>(&mut self, world: &WorldComponent) -> Option<Vec<T>> {
        let ti = TypeId::of::<T>();

        match self.components.remove(&ti) {
            Some(vec) => {
                let mut set = self.components.keys().cloned().collect::<BTreeSet<TypeId>>();
                set.insert(ti);

                world.change(
                    set,
                    self as *const _,
                    &ti,
                    false
                );

                Some(vec.into_iter().map(|x| *x.downcast::<T>().unwrap()).collect())
            },
            _ => None
        }
    }

    pub fn remove_all_unchecked<T: 'static>(&mut self, world: &WorldComponent) -> Vec<T> {
        let ti  = TypeId::of::<T>();

        world.change(self.components.keys().cloned().collect(), self as *const _, &ti, false);

        self.components.remove(&ti).unwrap()
            .into_iter()
            .map(|x| *x.downcast::<T>().unwrap())
            .collect()
    }
}

