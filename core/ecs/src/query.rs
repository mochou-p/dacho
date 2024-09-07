// dacho/core/ecs/src/query.rs

use core::any::TypeId;
use std::marker::PhantomData;

use super::entity::{Entity, EntityComponents};

pub struct Query<'entity, T: QueryT> {
    pub(crate) entities: Vec<&'entity Entity>,
               pd:       PhantomData<T>
}

impl<'entity, T: QueryT> Query<'entity, T> {
    pub(crate) fn new() -> Self {
        Self { entities: vec![], pd: PhantomData }
    }

    pub(crate) fn add(&mut self, entity: &'entity Entity) {
        self.entities.push(entity);
    }

    pub fn single(self) -> &'entity Entity {
        self.entities[0]
    }

    pub fn all(self) -> Vec<&'entity Entity> {
        self.entities
    }
}

pub trait QueryT: Sized {
    fn check(map: &EntityComponents) -> bool;
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

impl_query_t!(A);
impl_query_t!(A, B);
impl_query_t!(A, B, C);
impl_query_t!(A, B, C, D);
impl_query_t!(A, B, C, D, E);
impl_query_t!(A, B, C, D, E, F);
impl_query_t!(A, B, C, D, E, F, G);
impl_query_t!(A, B, C, D, E, F, G, H);
impl_query_t!(A, B, C, D, E, F, G, H, I);
impl_query_t!(A, B, C, D, E, F, G, H, I, J);
impl_query_t!(A, B, C, D, E, F, G, H, I, J, K);
impl_query_t!(A, B, C, D, E, F, G, H, I, J, K, L);

