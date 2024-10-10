// dacho/core/ecs/src/query.rs

use alloc::rc::{Rc, Weak};
use core::{any::TypeId, cell::RefCell, marker::PhantomData};

use super::entity::{Entity, EntityComponents};

use dacho_log::fatal;


pub struct Query<T>
where
    T: QueryT
{
    entities: Vec<Weak<Entity>>,
    pd:       PhantomData<T>
}

impl<T> Query<T>
where
    T: QueryT
{
    #[expect(clippy::new_without_default, reason = "Query is explicit")]
    pub fn new() -> Self {
        Self { entities: vec![], pd: PhantomData }
    }

    pub fn add(&mut self, entity: Weak<Entity>) {
        self.entities.push(entity);
    }

    pub fn one(&self) -> T::Components {
        if let Some(strong) = self.entities[0].upgrade() {
            return T::get(&strong);
        }

        fatal!("Weak<Entity> error");
    }

    pub fn all(&self) -> Vec<T::Components> {
        let mut components = vec![];

        for entity in &self.entities {
            if let Some(strong) = entity.upgrade() {
                components.push(T::get(&strong));
            } else {
                fatal!("QueryT::get error");
            }
        }

        components
    }

    pub fn entity(&self) -> Rc<Entity> {
        if let Some(strong) = self.entities[0].upgrade() {
            return strong;
        }

        fatal!("Weak<Entity> error");
    }

    pub fn entities(&self) -> Vec<Rc<Entity>> {
        let mut entities = Vec::with_capacity(self.entities.len());

        for entity in &self.entities {
            if let Some(strong) = entity.upgrade() {
                entities.push(strong);
            } else {
                fatal!("Weak<Entity> error");
            }
        }

        entities
    }
}

pub trait QueryT: Sized {
    type Components;

    fn check(map: &EntityComponents) -> bool;
    fn get(entity: &Entity) -> Self::Components;
}

pub trait QueryTuple: Sized {
    fn get_queries(entities: &[Rc<Entity>]) -> Option<Self>;
}

pub trait QueryFn<T>
where
    T: QueryTuple
{
    fn get_queries(&self, entities: &[Rc<Entity>]) -> Option<T>;
    fn run(&mut self, queries: T);
}

impl<T, F> QueryFn<T> for F
where
    T: QueryTuple,
    F: FnMut(T) // FnMut instead of Fn for now, until new built-ins in dacho-app
{
    fn get_queries(&self, entities: &[Rc<Entity>]) -> Option<T> {
        T::get_queries(entities)
    }

    fn run(&mut self, queries: T) {
        self(queries);
    }
}

macro_rules! impl_query_t {
    ($($t:tt),+) => {
        impl<$($t,)+> QueryT for ($($t,)+)
        where
            $($t: 'static,)+
        {
            type Components = ($(Rc<RefCell<$t>>,)+);

            fn check(map: &EntityComponents) -> bool {
                $(map.contains_key(&TypeId::of::<$t>()) &&)+ true
            }

            fn get(entity: &Entity) -> Self::Components {
                ($({
                    if let Some(component) = entity.get_component::<$t>() {
                        component
                    } else {
                        fatal!("Entity.get_component error");
                    }
                },)+)
            }
        }
    };
}

macro_rules! impl_query_tuple {
    ($($t:tt),+) => {
        impl<$($t,)+> QueryTuple for ($(Query<$t>,)+)
        where
            $($t: QueryT,)+
        {
            fn get_queries(entities: &[Rc<Entity>]) -> Option<Self> {
                Some(($({
                    let mut query = Query::<$t>::new();

                    for entity in entities {
                        if $t::check(&entity.components) {
                            query.add(Rc::downgrade(entity));
                        }
                    }

                    if query.entities.is_empty() {
                        return None;
                    }

                    query
                },)+))
            }
        }
    };
}

macro_rules! alphabet_impl {
    ($($i:tt),+) => {
        $(
            $i!(A);
            $i!(A, B);
            $i!(A, B, C);
            $i!(A, B, C, D);
            $i!(A, B, C, D, E);
            $i!(A, B, C, D, E, F);
            $i!(A, B, C, D, E, F, G);
            $i!(A, B, C, D, E, F, G, H);
            $i!(A, B, C, D, E, F, G, H, I);
            $i!(A, B, C, D, E, F, G, H, I, J);
            $i!(A, B, C, D, E, F, G, H, I, J, K);
            $i!(A, B, C, D, E, F, G, H, I, J, K, L);
        )+
    };
}

alphabet_impl!(impl_query_t, impl_query_tuple);

