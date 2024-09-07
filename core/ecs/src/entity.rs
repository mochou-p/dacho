// dacho/core/ecs/src/entity.rs

use core::any::{Any, TypeId};
use std::collections::HashMap;

pub type EntityComponents = HashMap<TypeId, Box<dyn Any>>;

pub struct Entity {
    pub(crate) components: EntityComponents
}

impl Entity {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { components: HashMap::new() }
    }
}

pub trait Tuple {
    fn insert_into(self, map: &mut EntityComponents);
}

macro_rules! impl_t {
    ($($i:tt $t:tt),+) => {
        impl<$($t: 'static,)+> Tuple for ($($t,)+) {
            fn insert_into(self, map: &mut EntityComponents) {
                $(map.insert(TypeId::of::<$t>(), Box::new(self.$i));)+
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

