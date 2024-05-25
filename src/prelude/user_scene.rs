// dacho/src/prelude/user_scene.rs

use super::shapes::Object;

pub struct UserScene {
    pub objects: Vec<Object>
}

impl UserScene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Object) -> &mut Self {
        self.objects.push(object);

        self
    }
}

impl Default for UserScene {
    fn default() -> Self {
        Self::new()
    }
}

