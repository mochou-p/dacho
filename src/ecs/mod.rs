// dacho/src/ecs/mod.rs

// modules
pub mod component;
pub mod entity;
pub mod world;

// pub use
pub use {
    std::any::Any,
    component::Component,
    world::{Id, World}
};

