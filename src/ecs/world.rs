// dacho/src/ecs/world.rs

// std
use std::collections::HashMap;

// super
use super::entity::Entity;

pub struct World {
    entities: HashMap<u64, Entity>,
    id:       u64
}

impl World {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { entities: HashMap::new(), id: 0 }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn spawn(&mut self, name: &str) -> (u64, &mut Entity) {
        let id = self.id;
        self.id += 1;

        self.entities.insert(id, Entity::new(name));

        (id, self.get(id).expect("unexpected HashMap error"))
    }

    pub fn get(&mut self, id: u64) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn debug(&self) {
        println!("-- World ----------------------");

        for (k, v) in &self.entities {
            print!("{k}: {}", v.name);

            if v.components.is_empty() {
                println!(",");
            } else {
                print!(" {{\n  components: {{ ");

                for c in &v.components {
                    print!("{}, ", c.name());
                }

                println!("}}\n}},");
            }
        }

        println!("-------------------------------");
    }
}

