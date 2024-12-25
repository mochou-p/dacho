// dacho/core/ecs/src/component.rs

use std::collections::HashMap;

use super::entity::EntityIndex;


pub(crate) type ComponentId      = u32;
pub(crate) type ComponentMask    = u32;
           type ComponentIndex   = usize;
pub(crate) type Components       = HashMap<ComponentId, Vec<(EntityIndex, Box<dyn Component>)>>;
pub(crate) type ComponentIndices = HashMap<ComponentId, Vec<ComponentIndex>>;

pub trait Component {
    fn id() -> ComponentId where Self: Sized;
}

pub trait ComponentGroup {
    fn mask() -> ComponentMask;

    fn insert_and_into_indices(self, owner: EntityIndex, map: &mut Components) -> ComponentIndices;
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

    #[expect(clippy::unwrap_used, reason = "never None")]
    #[must_use]
    fn insert_and_into_indices(self, owner: EntityIndex, map: &mut Components) -> ComponentIndices {
        let     id      = C::id();
        let mut indices = HashMap::from([(id, Vec::with_capacity(1))]);

        let vec = map.entry(id)
            .or_insert(Vec::with_capacity(16));

        vec.push((owner, Box::new(self)));

        indices.get_mut(&id).unwrap().push(vec.len() - 1);

        indices
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

            #[must_use]
            fn insert_and_into_indices(self, owner: EntityIndex, map: &mut Components) -> ComponentIndices {
                let ids = [$($ty::id()),+];

                let mut indices = HashMap::from([
                    $((ids[$i], Vec::with_capacity(1))),+
                ]);

                $(
                    let vec = map.entry(ids[$i])
                        .or_insert(Vec::with_capacity(16));

                    vec.push((owner, Box::new(self.$i)));

                    indices.get_mut(&ids[$i]).unwrap().push(vec.len() - 1);
                )+

                indices
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

