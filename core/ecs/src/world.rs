// dacho/core/ecs/src/world.rs

use {
    core::any::TypeId,
    alloc::collections::BTreeSet,
    std::collections::HashMap
};

use super::{entity::Entity, query::QueryT};


#[non_exhaustive]
pub struct World {
    pub entities:      HashMap<BTreeSet<TypeId>, Vec<Entity>>,
    pub query_matches: HashMap<BTreeSet<TypeId>, Vec<BTreeSet<TypeId>>>
}

impl World {
    #[expect(clippy::new_without_default, reason = "default would be empty")]
    pub fn new() -> Self {
        Self { entities: HashMap::new(), query_matches: HashMap::new() }
    }

    pub fn spawn<T: QueryT + 'static>(&mut self, tuple: T) {
        self.entities
            .entry(T::get_set())
            .or_insert(Vec::with_capacity(1))
            .push(Entity::new(tuple));
    }

    pub(crate) fn first_match(&self, set: BTreeSet<TypeId>) -> &Entity {
        &self.entities
            [&self.query_matches[&set][0]]
            [0]
    }

    #[expect(clippy::unwrap_used, reason = "this is only called from `Query`, which only exist when the data exists")]
    pub(crate) fn first_mut_match(&mut self, set: BTreeSet<TypeId>) -> &mut Entity {
        &mut self.entities
            .get_mut(
                &self.query_matches
                    .get_mut(&set)
                    .unwrap()
                    [0]
            )
            .unwrap()
            [0]
    }

    pub(crate) fn matches_iter(&self, set: BTreeSet<TypeId>) -> impl Iterator<Item = &Entity> {
        self.query_matches
            [&set]
            .iter()
            .map(|query_match| &self.entities[query_match])
            .flat_map(|entities| entities.iter())
    }

    #[expect(clippy::unwrap_used, reason = "this is only called from `Query`, which only exist when the data exists")]
    pub(crate) unsafe fn matches_iter_mut(&mut self, set: BTreeSet<TypeId>) -> impl Iterator<Item = &mut Entity> {
        self.query_matches
            .get_mut(&set)
            .unwrap()
            .iter_mut()
            .map(|query_match| &mut *(self.entities.get_mut(query_match).unwrap() as *mut Vec<Entity>))
            .flat_map(|entities| entities.iter_mut())
    }
}

