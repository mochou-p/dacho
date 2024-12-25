// dacho/core/ecs/src/system.rs

use super::query::QueryMarker;


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
{}

impl<F> System<()> for F
where
    F: Fn()
{}

macro_rules! impl_system_for_tuple {
    ($($ty:tt),+) => {
        impl<T, $($ty),+> System<($($ty,)+)> for T
        where
            T: Fn($($ty),+),
            $($ty: Argument + QueryMarker),+
        {}
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

