// dacho/core/ecs/src/component.rs

use std::collections::HashMap;

use super::entity::EntityIndex;


pub type ComponentId      = u32;
pub type ComponentMask    = u32;
pub type ComponentIndex   = (ComponentMask, usize);
pub type ComponentIndices = HashMap<ComponentMask, Vec<usize>>;
pub type Components       = HashMap<ComponentMask, Vec<(EntityIndex, Box<dyn ComponentGroup>)>>;

pub trait Component {
    fn id() -> ComponentId;
}

pub trait ComponentGroup {
    fn mask() -> ComponentMask where Self: Sized;

    fn insert_and_into_index(self, owner: EntityIndex, map: &mut Components) -> ComponentIndex
    where
        Self: Sized + 'static
    {
        let mask = Self::mask();
        let vec  = map.entry(mask)
            .or_insert(Vec::with_capacity(16));

        vec.push((owner, Box::new(self)));

        (mask, vec.len() - 1)
    }
}

impl<C> ComponentGroup for C
where
    C: Component + 'static
{
    #[inline]
    #[must_use]
    fn mask() -> ComponentMask {
        1 << C::id()
    }
}

macro_rules! impl_component_group_for_tuple {
    ($($i:tt $ty:tt),+) => {
        impl<$($ty),+> ComponentGroup for ($($ty,)+)
        where
            $($ty: Component + 'static),+
        {
            #[inline]
            #[must_use]
            fn mask() -> ComponentMask {
                0 $(| 1 << $ty::id())+
            }
        }
    }
}

impl_component_group_for_tuple!(0 A);
impl_component_group_for_tuple!(0 A, 1 B);
impl_component_group_for_tuple!(0 A, 1 B, 2 C);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
impl_component_group_for_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);

