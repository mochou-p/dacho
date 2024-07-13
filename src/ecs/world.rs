// dacho/src/ecs/world.rs

// std
use std::collections::HashMap;

// super
use super::entity::Entity;

pub struct World {
    entities: HashMap<u64, Entity>
}

