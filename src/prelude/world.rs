// dacho/src/prelude/world.rs

use super::{
    shapes::Object,
    dacho_main
};

pub struct World {
    pub objects: Vec<Object>
}

#[allow(clippy::new_without_default)]
impl World {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, objects: &[Object]) -> &mut Self {
        self.objects.extend_from_slice(objects);

        self
    }

    #[inline]
    pub fn run(&self) {
        dacho_main(self)
            .expect("failed to run dacho_main");
    }
}

