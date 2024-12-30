// dacho/core/ecs/src/entity.rs

use std::collections::HashMap;

use super::component::{ComponentMask, ComponentIndices, Components, ComponentGroup};


pub(crate) type Entities    = HashMap<ComponentMask, Vec<Entity>>;
pub(crate) type EntityIndex = usize;

#[non_exhaustive]
pub struct Entity {
    _component_indices: ComponentIndices
}

impl Entity {
    #[inline]
    #[must_use]
    pub(crate) fn from<CG>(component_group: CG, self_i: EntityIndex, components: &mut Components) -> Self
    where
        CG: ComponentGroup + 'static
    {
        let (mask, index) = component_group.insert_and_into_index(self_i, components);
        let mut indices   = Vec::with_capacity(16);
        indices.push(index);

        Self {
            _component_indices: ComponentIndices::from([
                (mask, indices)
            ])
        }
    }
}

