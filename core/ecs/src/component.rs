// dacho/core/ecs/src/component.rs

use {
    core::any::type_name,
    std::collections::{HashMap, HashSet}
};

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
    fn validate() where Self: Sized;

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
    fn validate() {}

    #[inline]
    #[must_use]
    fn mask() -> ComponentMask {
        1 << C::id()
    }
}

macro_rules! impl_component_group_for_tuple {
    ($($ty:tt),+) => {
        impl<$($ty),+> ComponentGroup for ($($ty,)+)
        where
            $($ty: Component + 'static),+
        {
            // temp: doing this again in mask, but its temp whatever
            //       later make this stuff const, and there you can
            //       return mask from {} like:
            //         const MASK: u32 = { /* blablabla */ mask };
            #[inline]
            fn validate() {
                // cause mask thought its not used (rustc what)
                let mut set = HashSet::with_capacity(12);

                $(assert!(
                    set.insert($ty::id()),
                    "duplicate component types are not allowed in one Entity ({})",
                    type_name::<$ty>()
                );)+
            }

            #[inline]
            #[must_use]
            fn mask() -> ComponentMask {
                0 $(| 1 << $ty::id())+
            }
        }
    }
}

impl_component_group_for_tuple!(A);
impl_component_group_for_tuple!(A, B);
impl_component_group_for_tuple!(A, B, C);
impl_component_group_for_tuple!(A, B, C, D);
impl_component_group_for_tuple!(A, B, C, D, E);
impl_component_group_for_tuple!(A, B, C, D, E, F);
impl_component_group_for_tuple!(A, B, C, D, E, F, G);
impl_component_group_for_tuple!(A, B, C, D, E, F, G, H);
impl_component_group_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_component_group_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_component_group_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_component_group_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

