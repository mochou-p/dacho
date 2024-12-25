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
    pub(crate) fn from<CG>(component_group: CG, index: EntityIndex, components: &mut Components) -> Self
    where
        CG: ComponentGroup
    {
        Self { _component_indices: component_group.insert_and_into_indices(index, components) }
    }
}

