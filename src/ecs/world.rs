// dacho/src/ecs/world.rs

// std
use std::collections::HashMap;

// super
use super::entity::Entity;

pub struct World {
    entities: HashMap<u64, Entity>,
    counter:  u64
}

impl World {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { entities: HashMap::new(), counter: 0 }
    }

    pub fn add(&mut self, name: &str) -> u64 {
        self.entities.insert(self.counter, Entity::new(name));

        self.counter += 1;

        self.counter - 1
    }

    pub fn get(&mut self, id: u64) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn debug(&self) {
        println!("-- World --");
        for (k, v) in &self.entities {
            println!("{k}: {}", v.name);
        }
        println!("-----------");
    }
}

