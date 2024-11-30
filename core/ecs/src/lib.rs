// dacho/core/ecs/src/lib.rs

#![feature(const_type_id)]

extern crate alloc;

pub mod entity;
pub mod query;
pub mod world;

use core::any::TypeId;

use dacho_mesh_c::MeshComponent;

pub use {query::Query, world::{World, WorldComponent}};


pub const MESH_TI: TypeId = TypeId::of::<MeshComponent>();

