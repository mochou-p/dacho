// dacho/core/ecs/src/world.rs

use super::{component::{Components, ComponentGroup}, entity::{Entities, Entity}};


#[non_exhaustive]
pub struct World {
    pub entities:   Entities,
        components: Components
}

#[expect(clippy::new_without_default, reason = "would be empty")]
impl World {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entities:   Entities  ::new(),
            components: Components::new()
        }
    }

    #[inline]
    pub fn add<CG>(&mut self, component_group: CG)
    where
        CG: ComponentGroup
    {
        let entity = Entity::from(component_group, self.entities.len(), &mut self.components);

        self.entities.entry(CG::mask())
            .or_insert(Vec::with_capacity(16))
            .push(entity);
    }
}

