// dacho/core/ecs/src/world.rs

use {
    core::any::TypeId,
    alloc::collections::{btree_map::Entry::{Occupied as BtmOccupied, Vacant as BtmVacant}, BTreeMap, BTreeSet},
    std::collections::{hash_map::Entry::{Occupied as HmOccupied, Vacant as HmVacant}, HashMap}
};

use super::{entity::Entity, query::QueryT};


pub struct WorldComponent {
    world: *mut World
}

impl WorldComponent {
    /// # Safety
    ///
    /// this should only be called by App, and only once
    /// it is only pub because of the current crate structure
    pub unsafe fn new(world: *mut World) -> Self {
        Self { world }
    }

    pub(crate) fn change(&self, set: BTreeSet<TypeId>, ptr: *const Entity, ti: &TypeId, added: bool) {
        // SAFETY: raw pointer dereference
        let map   = unsafe { &mut (*self.world).moves };
        // SAFETY: raw pointer dereference, pointer arithmetic
        let index = unsafe { ptr.offset_from((*self.world).entities[&set].as_ptr()) };

        #[expect(clippy::shadow_reuse, reason = "intended")]
        match map.entry(set) {
            HmOccupied(mut entry) => {
                match entry.get_mut().entry(index as usize) {
                    BtmOccupied(mut entry) => {
                        match entry.get_mut().entry(*ti) {
                            HmOccupied(entry) => if *entry.get() != added {
                                entry.remove();
                            },
                            HmVacant(entry) => {
                                entry.insert(added);
                            }
                        }
                    },
                    BtmVacant(entry) => {
                        entry.insert(HashMap::from([(*ti, added)]));
                    }
                }
            },
            HmVacant(entry) => {
                entry.insert(BTreeMap::from([(index as usize, HashMap::from([(*ti, added)]))]));
            }
        }
    }
}

// TODO: SoA, branchless
//       or magic, i could make a derive proc-macro, and if it can store things,
//       magic could help with contiguity (since the SoA problem is not knowing all user types)
//       but can macros even expand in/into structs?
#[non_exhaustive]
pub struct World {
    pub entities:      HashMap<BTreeSet<TypeId>, Vec<Entity>>,
    pub query_matches: HashMap<BTreeSet<TypeId>, Vec<BTreeSet<TypeId>>>,
        moves:         HashMap<BTreeSet<TypeId>, BTreeMap<usize, HashMap<TypeId, bool>>>,
    pub changed_sets:  (Vec<BTreeSet<TypeId>>, Vec<BTreeSet<TypeId>>)
}

impl World {
    #[expect(clippy::new_without_default, reason = "default would be empty")]
    pub fn new() -> Self {
        Self {
            entities:      HashMap::new(),
            query_matches: HashMap::new(),
            moves:         HashMap::new(),
            changed_sets:  (vec![], vec![])
        }
    }

    pub fn spawn<T: QueryT + 'static>(&mut self, tuple: T) {
        self.entities
            .entry(T::get_set())
            .or_insert(Vec::with_capacity(1))
            .push(Entity::new(tuple));
    }

    #[expect(clippy::unwrap_used, reason = "guarded")]
    pub fn check_moves(&mut self) {
        if self.moves.is_empty() {
            return;
        }

        for (set, btm) in &self.moves {
            let mut indices   = HashMap::with_capacity(btm.len());

            for (index, map) in btm {
                let index_clone = *match indices.get(index) {
                    Some(new_index) => new_index,
                    _               => index
                };

                let     entities = self.entities.get_mut(set).unwrap();
                let     entity   = entities.swap_remove(index_clone);
                let mut new_set  = set.clone();

                indices.insert(entities.len(), index_clone);

                for (ti, added) in map {
                    if *added {
                        new_set.insert(*ti);
                    } else {
                        new_set.remove(ti);
                    }
                }

                if entities.is_empty() {
                    self.entities.remove(set);
                    self.changed_sets.1.push(set.clone());
                }

                match self.entities.entry(new_set.clone()) {
                    HmOccupied(mut entry) => { entry.get_mut().push(entity); },
                    HmVacant(entry) => {
                        entry.insert(vec![entity]);
                        self.changed_sets.0.push(new_set.clone());
                    }
                }
            }
        }

        self.moves.clear();

        for (set, matches) in &mut self.query_matches {
            for removed_set in &self.changed_sets.1 {
                if set.is_subset(removed_set) {
                    matches.remove(matches.iter().position(|x| x == removed_set).unwrap());
                }
            }

            for new_set in &self.changed_sets.0 {
                if set.is_subset(new_set) {
                    matches.push(new_set.clone());
                }
            }
        }
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

