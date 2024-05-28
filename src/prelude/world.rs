// dacho/src/prelude/world.rs

use super::shapes::Object;

pub struct World {
    pub objects: Vec<Object>
}

impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Object) -> &mut Self {
        self.objects.push(object);

        self
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

