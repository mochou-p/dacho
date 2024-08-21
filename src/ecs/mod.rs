// dacho/src/ecs/mod.rs

// modules
pub mod component;
pub mod entity;
pub mod system;
pub mod world;

// pub use
pub use {
    component::Component,
    world::{Id, State, World}
};

