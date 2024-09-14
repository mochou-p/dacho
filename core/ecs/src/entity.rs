// dacho/core/ecs/src/entity.rs

use alloc::rc::Rc;
use core::{any::{Any, TypeId}, cell::RefCell};
use std::collections::HashMap;

pub type EntityComponents = HashMap<TypeId, Vec<Box<dyn Any>>>;

pub struct Entity {
    pub(crate) components: EntityComponents
}

impl Entity {
    #[expect(clippy::new_without_default, reason = "default would just be empty")]
    pub fn new() -> Self {
        Self { components: HashMap::new() }
    }

    pub fn get_component<T>(&self) -> Option<Rc<RefCell<T>>>
    where
        T: 'static
    {
        if let Some(any_components) = self.components.get(&TypeId::of::<T>()) {
            if let Some(any_component) = any_components.first() {
                if let Some(component) = any_component.downcast_ref::<Rc<RefCell<T>>>() {
                    return Some(Rc::clone(component));
                }
            }
        }

        None
    }

    pub fn get_components<T>(&self) -> Option<Vec<Rc<RefCell<T>>>>
    where
        T: 'static
    {
        if let Some(any_components) = self.components.get(&TypeId::of::<T>()) {
            let mut components = Vec::with_capacity(any_components.len());

            for any_component in any_components {
                if let Some(component) = any_component.downcast_ref::<Rc<RefCell<T>>>() {
                    components.push(Rc::clone(component));
                }
            }

            return
                if components.is_empty() {
                    None
                } else {
                    Some(components)
                };
        }

        None
    }
}

pub trait Tuple {
    fn insert_into(self, map: &mut EntityComponents);
}

macro_rules! impl_t {
    ($($i:tt $t:tt),+) => {
        impl<$($t,)+> Tuple for ($($t,)+)
        where
            $($t: 'static,)+
        {
            fn insert_into(self, map: &mut EntityComponents) {
                $(
                    map
                        .entry(TypeId::of::<$t>())
                        .or_insert_with(|| Vec::with_capacity(1))
                        .push(Box::new(Rc::new(RefCell::new(self.$i))));
                )+
            }
        }
    };
}

impl_t!(0 A);
impl_t!(0 A, 1 B);
impl_t!(0 A, 1 B, 2 C);
impl_t!(0 A, 1 B, 2 C, 3 D);
impl_t!(0 A, 1 B, 2 C, 3 D, 4 E);
impl_t!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
impl_t!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
impl_t!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
impl_t!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
impl_t!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
impl_t!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
impl_t!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);

