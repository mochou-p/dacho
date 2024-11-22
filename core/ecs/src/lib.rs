// dacho/core/ecs/src/lib.rs

extern crate alloc;

pub mod entity;
pub mod query;
pub mod world;

pub use {query::Query, world::{World, WorldComponent}};

