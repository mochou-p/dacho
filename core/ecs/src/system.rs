// dacho/core/ecs/src/system.rs

use std::collections::HashMap;

use super::{
    component::ComponentMask,
    query::QueryMarker,
    world::World
};


    type   StoredSystem = Box<dyn Fn()>;
pub type    SystemIndex = usize;
pub type        Systems = [Vec<SystemT>                             ; 2];
pub type  SystemQueries = [HashMap<ComponentMask, Vec<SystemIndex>> ; 2];

pub struct SystemT {
    pub total:    u8,
    pub ready:    u8,
    pub function: StoredSystem
}

impl SystemT {
    pub fn from_and_insert_into<S, A>(system: S, world: &mut World)
    where
        S: System<A>,
        A: Arguments
    {
        let     masks   = S::masks();
        let     total   = masks.len() as u8;
        let mut ready   = 0;

        masks.iter()
            .for_each(|mask| {
                ready += world.components
                    .contains_key(mask)
                    as u8;
            });

        let world_ptr = world as *mut _;
        let status    = (ready == total) as usize;
        let vec       = &mut world.systems[status];
        let i         = vec.len();

        masks.into_iter()
            .for_each(|mask| {
                world.queries[status]
                    .entry(mask)
                    .or_insert(Vec::with_capacity(32))
                    .push(i);
            });

        vec.push(SystemT {
            total,
            ready,
            function: system.shapeshift(world_ptr)
        });
    }
}

pub trait Argument {}

impl Argument for () {}

pub trait Arguments {}

impl Arguments for () {}

macro_rules! impl_arguments_for_tuple {
    ($($ty:tt),+) => {
        impl<$($ty),+> Arguments for ($($ty,)+)
        where
            $($ty: Argument),+
        {}
    }
}

pub trait System<A>
where
    A: Arguments
{
    fn masks() -> Vec<ComponentMask>;

    fn shapeshift(self, world: *mut World) -> StoredSystem;
}

impl<F> System<()> for F
where
    F: Fn() + 'static
{
    #[inline]
    #[must_use]
    fn masks() -> Vec<ComponentMask> {
        vec![]
    }

    #[inline]
    #[must_use]
    fn shapeshift(self, _: *mut World) -> StoredSystem {
        Box::new(move || { self(); })
    }
}

macro_rules! impl_system_for_tuple {
    ($($ty:tt),+) => {
        impl<T, $($ty),+> System<($($ty,)+)> for T
        where
            T: Fn($(&$ty),+) + 'static,
            $($ty: Argument + QueryMarker),+
        {
            #[inline]
            #[must_use]
            fn masks() -> Vec<ComponentMask> {
                vec![$($ty::mask()),+]
            }

            #[inline]
            #[must_use]
            fn shapeshift(self, world: *mut World) -> StoredSystem {
                Box::new(move || {
                    self($(&$ty::pes(world)),+);
                })
            }
        }
    }
}

impl_arguments_for_tuple!(A);
impl_arguments_for_tuple!(A, B);
impl_arguments_for_tuple!(A, B, C);
impl_arguments_for_tuple!(A, B, C, D);
impl_arguments_for_tuple!(A, B, C, D, E);
impl_arguments_for_tuple!(A, B, C, D, E, F);
impl_arguments_for_tuple!(A, B, C, D, E, F, G);
impl_arguments_for_tuple!(A, B, C, D, E, F, G, H);
impl_arguments_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_arguments_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_arguments_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_arguments_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

impl_system_for_tuple!(A);
impl_system_for_tuple!(A, B);
impl_system_for_tuple!(A, B, C);
impl_system_for_tuple!(A, B, C, D);
impl_system_for_tuple!(A, B, C, D, E);
impl_system_for_tuple!(A, B, C, D, E, F);
impl_system_for_tuple!(A, B, C, D, E, F, G);
impl_system_for_tuple!(A, B, C, D, E, F, G, H);
impl_system_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_system_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_system_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_system_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

