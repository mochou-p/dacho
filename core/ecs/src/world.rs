// dacho/core/ecs/src/world.rs

use std::collections::HashMap;

use super::{
    component::{Components, Component, ComponentGroup},
    entity::{Entities, Entity},
    system::{SystemIndex, Systems, SystemQueries, SystemT, Arguments, System}
};


type IndexSwaps = HashMap<SystemIndex, SystemIndex>;

const WAITING: usize = 0;
const READY:   usize = 1;

pub struct WorldComponent {
    world: *mut World
}

impl WorldComponent {
    #[inline]
    #[must_use]
    pub fn new(world: *mut World) -> Self {
        Self { world }
    }

    #[inline]
    pub fn add<CG>(&self, component_group: CG)
    where
        CG: ComponentGroup + 'static
    {
        // SAFETY: pointer always valid
        unsafe { (*self.world).add(component_group); }
    }
}

impl Component for WorldComponent {
    #[inline]
    #[must_use]
    fn id() -> u32 { 0 }
}

#[non_exhaustive]
pub struct World {
    pub entities:    Entities,
    pub components:  Components,
    pub systems:     Systems,
    pub queries:     SystemQueries,
        index_swaps: IndexSwaps
}

#[expect(clippy::new_without_default, reason = "would be empty")]
impl World {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            entities:    Entities  ::new(),
            components:  Components::new(),
            systems:     [Vec::with_capacity(32), Vec::with_capacity(32)],
            queries:     [HashMap::new(),         HashMap::new()        ],
            index_swaps: IndexSwaps::new()
        }
    }

    pub fn add<CG>(&mut self, component_group: CG)
    where
        CG: ComponentGroup + 'static
    {
        CG::validate();

        let entity = Entity::from(component_group, self.entities.len(), &mut self.components);
        let mask   = CG::mask();

        self.queries[WAITING]
            .entry(mask)
            .and_modify(|waiting_query_indices| {
                for index in waiting_query_indices.drain(..) {
                    let i = *self.index_swaps
                        .get(&index)
                        .unwrap_or(&index);

                    let system = &mut self.systems[WAITING][i];

                    system.ready += 1;

                    if system.ready == system.total {
                        let last         = self.systems[WAITING].len() - 1;
                        let ready_system = self.systems[WAITING].swap_remove(i);
                        self.index_swaps.insert(last, i);
                        self.systems[READY].push(ready_system);
                    }
                }
            });

        self.entities
            .entry(mask)
            .or_insert(Vec::with_capacity(16))
            .push(entity);
    }

    #[inline]
    pub fn add_system<S, A>(&mut self, system: S)
    where
        S: System<A>,
        A: Arguments
    {
        SystemT::from_and_insert_into::<S, A>(system, self);
    }

    #[inline]
    pub fn run_systems(&self) {
        for system in &self.systems[READY] {
            (system.function)();
        }
    }
}

