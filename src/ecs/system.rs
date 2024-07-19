// dacho/src/ecs/system.rs

// super
use super::world::World;

pub type  StartSystem = Box<dyn FnOnce(&mut World)>;
pub type UpdateSystem = Box<dyn Fn    (&mut World)>;

