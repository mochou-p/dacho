// dacho/core/ecs/src/world.rs

use {
    core::any::TypeId,
    alloc::collections::{btree_map::Entry::{Occupied as BtmOccupied, Vacant as BtmVacant}, BTreeMap, BTreeSet},
    std::collections::{hash_map::Entry::{Occupied as HmOccupied, Vacant as HmVacant}, HashMap, HashSet}
};

use super::{entity::Entity, query::QueryT};

use dacho_mesh_c::MeshComponent;


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

    pub fn insert<T: QueryT + 'static>(&self, tuple: T) {
        // SAFETY: raw pointer dereference
        unsafe { (*self.world).insert(tuple); }
    }

    pub(crate) fn change(&self, set: BTreeSet<TypeId>, ptr: *const Entity, ti: &TypeId, added: bool) {
        // SAFETY: raw pointer dereference
        let map   = unsafe { &mut (*self.world).moves };
        // SAFETY: raw pointer dereference, pointer arithmetic
        let index = unsafe { ptr.offset_from((*self.world).entities[&set].0.as_ptr()) };

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

    pub fn mesh_updater(&self) -> impl Fn(*const MeshComponent) + use<'_> {
        |ptr| {
            // SAFETY: raw pointer dereference
            unsafe { (*self.world).updated_meshes.insert(ptr); }

            // SAFETY: raw pointer dereference
            unsafe { (*self.world).meshes_updated = true; }
        }
    }
}

#[non_exhaustive]
pub struct World {
    pub entities:        HashMap<BTreeSet<TypeId>, (Vec<Entity>, Vec<Entity>)>,
    pub query_matches:   HashMap<BTreeSet<TypeId>, Vec<BTreeSet<TypeId>>>,
        moves:           HashMap<BTreeSet<TypeId>, BTreeMap<usize, HashMap<TypeId, bool>>>,
    pub changed_sets:    (Vec<BTreeSet<TypeId>>, Vec<BTreeSet<TypeId>>),
    pub meshes:          HashMap<u32, Vec<f32>>,
    pub meshes_updated:  bool,
        updated_meshes:  HashSet<*const MeshComponent>,
        mesh_id_counter: HashMap<u32, u32>
}

impl World {
    #[expect(clippy::new_without_default, reason = "default would be empty")]
    pub fn new() -> Self {
        Self {
            entities:        HashMap::new(),
            query_matches:   HashMap::new(),
            moves:           HashMap::new(),
            changed_sets:    (vec![], vec![]),
            meshes:          HashMap::new(),
            meshes_updated:  false,
            updated_meshes:  HashSet::new(),
            mesh_id_counter: HashMap::new()
        }
    }

    fn check_for_meshes<T: QueryT>(&mut self, tuple: &mut T) {
        let meshes = tuple.get_meshes(&mut self.mesh_id_counter);

        if !meshes.is_empty() {
            for (id, model_matrix) in meshes {
                self.meshes
                    .entry(id)
                    .and_modify(|vec| vec.extend(model_matrix))
                    .or_insert(model_matrix.to_vec());

                self.meshes_updated = true;
            }
        }
    }

    #[expect(clippy::unwrap_used, reason = "guarded")]
    pub fn get_meshes(&mut self) -> &HashMap<u32, Vec<f32>> {
        // SAFETY: raw pointer dereference, guarded in context by correct order of mesh work
        unsafe {
            for mesh in self.updated_meshes.drain() {
                let i = ((*mesh).nth * 16) as usize;

                self.meshes
                    .get_mut(&(*mesh).id)
                    .unwrap()
                    [i..i+16]
                    .copy_from_slice(&(*mesh).model_matrix.to_cols_array());
            }
        }

        self.meshes_updated = false;

        &self.meshes
    }

    pub fn spawn<T: QueryT + 'static>(&mut self, mut tuple: T) {
        self.check_for_meshes(&mut tuple);

        self.entities
            .entry(T::get_set())
            .or_insert((Vec::with_capacity(1), vec![]))
            .0.push(Entity::new(tuple));
    }

    fn insert<T: QueryT + 'static>(&mut self, mut tuple: T) {
        self.check_for_meshes(&mut tuple);

        let set = T::get_set();

        self.entities
            .entry(set.clone())
            .or_insert_with(|| {
                self.changed_sets.0.push(set);

                (Vec::with_capacity(1), Vec::with_capacity(1))
            })
            .1.push(Entity::new(tuple));
    }

    #[expect(clippy::unwrap_used, reason = "guarded")]
    pub fn check_moves(&mut self) {
        if self.moves.is_empty() {
            return;
        }

        for (set, btm) in &self.moves {
            let mut indices = HashMap::with_capacity(btm.len());

            for (index, map) in btm {
                let index_clone = *match indices.get(index) {
                    Some(new_index) => new_index,
                    _               => index
                };

                let     entities = &mut self.entities.get_mut(set).unwrap().0;
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
                    HmOccupied(mut entry) => { entry.get_mut().0.push(entity); },
                    HmVacant(entry) => {
                        entry.insert((vec![entity], vec![]));
                        self.changed_sets.0.push(new_set.clone());
                    }
                }
            }
        }

        self.moves.clear();
    }

    pub fn update_matches(&mut self) {
        for (set, matches) in &mut self.query_matches {
            for removed_set in &self.changed_sets.1 {
                if set.is_subset(removed_set) {
                    #[expect(clippy::unwrap_used, reason = "guarded")]
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

    pub fn move_new_entities(&mut self) {
        for entities in self.entities.values_mut() {
            if !entities.1.is_empty() {
                entities.0.append(&mut entities.1);
            }
        }
    }

    pub(crate) fn first_match(&self, set: BTreeSet<TypeId>) -> &Entity {
        &self.entities
            [&self.query_matches[&set][0]]
            .0[0]
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
            .0[0]
    }

    pub(crate) fn matches_iter(&self, set: BTreeSet<TypeId>) -> impl Iterator<Item = &Entity> {
        self.query_matches
            [&set]
            .iter()
            .map(|query_match| &self.entities[query_match].0)
            .flat_map(|entities| entities.iter())
    }

    #[expect(clippy::unwrap_used, reason = "this is only called from `Query`, which only exist when the data exists")]
    pub(crate) unsafe fn matches_iter_mut(&mut self, set: BTreeSet<TypeId>) -> impl Iterator<Item = &mut Entity> {
        self.query_matches
            .get_mut(&set)
            .unwrap()
            .iter_mut()
            .map(|query_match| &mut *(&mut self.entities.get_mut(query_match).unwrap().0 as *mut Vec<Entity>))
            .flat_map(|entities| entities.iter_mut())
    }
}

