// dacho/core/ecs/src/query.rs

use core::any::TypeId;
use std::{marker::PhantomData, rc::{Rc, Weak}};

use super::entity::{Entity, EntityComponents};

pub struct Query<T: QueryT> {
    entities: Vec<Weak<Entity>>,
    pd:       PhantomData<T>
}

impl<T> Query<T>
where
    T: QueryT
{
    pub(crate) fn new() -> Self {
        Self { entities: vec![], pd: PhantomData }
    }

    pub(crate) fn add(&mut self, entity: Weak<Entity>) {
        self.entities.push(entity);
    }

    pub fn single(self) -> Option<Rc<Entity>> {
        self.entities[0].upgrade()
    }

    pub fn all(self) -> Option<Vec<Rc<Entity>>> {
        let mut entities = Vec::with_capacity(self.entities.len());

        for weak_entity in self.entities {
            if let Some(entity) = weak_entity.upgrade() {
                entities.push(entity);
            } else {
                return None;
            }
        }

        Some(entities)
    }
}

pub trait QueryT {
    fn check(map: &EntityComponents) -> bool;
}

pub trait QueryTuple: Sized {
    fn get_queries(entities: &Vec<Rc<Entity>>) -> Option<Self>;
}

pub trait QueryFn<T> {
    fn get_queries(&self, entities: &Vec<Rc<Entity>>) -> Option<T>;
    fn call(&self, queries: T);
}

impl<T, F> QueryFn<T> for F
where
    T: QueryTuple,
    F: Fn(T)
{
    fn get_queries(&self, entities: &Vec<Rc<Entity>>) -> Option<T> {
        T::get_queries(entities)
    }

    fn call(&self, queries: T) {
        self(queries);
    }
}

macro_rules! impl_query_t {
    ($($t:tt),+) => {
        impl<$($t: 'static,)+> QueryT for ($($t,)+) {
            fn check(map: &EntityComponents) -> bool {
                $(map.contains_key(&TypeId::of::<$t>()) &&)+ true
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
            fn get_queries(entities: &Vec<Rc<Entity>>) -> Option<Self> {
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

