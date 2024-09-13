// dacho/core/ecs/src/world.rs

use std::rc::Rc;

use super::entity::{Entity, Tuple};

#[non_exhaustive]
pub struct World {
    pub entities: Vec<Rc<Entity>>
}

impl World {
    #[expect(clippy::new_without_default, reason = "default would just be empty")]
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    pub fn spawn<T>(&mut self, components: T)
    where
        T: Tuple
    {
        let mut entity = Entity::new();
        components.insert_into(&mut entity.components);

        self.entities.push(Rc::new(entity));
    }
}

