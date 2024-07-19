// dacho/src/ecs/system.rs

// super
use super::world::{State, World};

pub type  StartSystem = Box<dyn FnOnce(&mut World)>;
pub type UpdateSystem = Box<dyn Fn    (&mut World)>;

pub type  StateSystem = Box<dyn Fn    (&mut World, State, State)>;

