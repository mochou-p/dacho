// dacho/src/ecs/entity.rs

// super
use super::component::Component;

pub struct Entity {
    parent:     Option<u64>,
    children:   Vec<u64>,
    components: Vec<Box<dyn Component>>
}

