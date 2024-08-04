// dacho/src/prelude/mod.rs

// modules
pub        mod mesh;
pub(super) mod types;

// pub mod
pub use {
    super::{ecs::*, app::*},
    mesh::Mesh,
    types::*
};

