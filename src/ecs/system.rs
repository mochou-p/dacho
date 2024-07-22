// dacho/src/ecs/system.rs

// crates
use winit::{keyboard::KeyCode, event::ElementState};

// super
use super::world::{State, World};

pub type    StateSystem = Box<dyn Fn    (&mut World, State, State)>;

pub type    StartSystem = Box<dyn FnOnce(&mut World)>;
pub type   UpdateSystem = Box<dyn Fn    (&mut World)>;

pub type KeyboardSystem = Box<dyn Fn    (&mut World, KeyCode, ElementState)>;

