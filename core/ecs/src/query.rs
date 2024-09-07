// dacho/core/ecs/src/query.rs

use core::any::TypeId;

use super::entity::EntityComponents;

pub trait Query: Sized {
    const ERR: &'static str = "guard Query::get with Query::check";

    fn check(map: &EntityComponents) -> bool;
    fn get(map: &mut EntityComponents) -> Self;
    fn return_to(self, map: &mut EntityComponents);
}

macro_rules! impl_query {
    ($($i:tt $t:tt),+) => {
        impl<$($t: 'static,)+> Query for ($($t,)+) {
            fn check(map: &EntityComponents) -> bool {
                $(map.contains_key(&TypeId::of::<$t>()) &&)+ true
            }

            fn get(map: &mut EntityComponents) -> Self {
                ($(*map.remove(&TypeId::of::<$t>()).unwrap().downcast::<$t>().unwrap(),)+)
            }

            fn return_to(self, map: &mut EntityComponents) {
                $(map.insert(TypeId::of::<$t>(), Box::new(self.$i));)+
            }
        }
    };
}

impl_query!(0 A);
impl_query!(0 A, 1 B);
impl_query!(0 A, 1 B, 2 C);
impl_query!(0 A, 1 B, 2 C, 3 D);
impl_query!(0 A, 1 B, 2 C, 3 D, 4 E);
impl_query!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
impl_query!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
impl_query!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
impl_query!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
impl_query!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
impl_query!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
impl_query!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);

