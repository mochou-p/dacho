// dacho/src/ecs/entity.rs

// super
use super::component::Component;

pub struct Entity {
    pub name:       String,
        parent:     Option<u64>,
        children:   Vec<u64>,
        components: Vec<Box<dyn Component>>
}

impl Entity {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), parent: None, children: vec![], components: vec![] }
    }
}

