// dacho/core/components/world/src/lib.rs

extern crate alloc;

use {
    alloc::rc::Weak,
    core::cell::{RefCell, RefMut}
};

use {
    dacho_ecs::World,
    dacho_log::fatal
};


#[expect(clippy::exhaustive_structs, reason = "reexported, but created by struct expression")]
pub struct WorldComponent {
    pub world: Weak<RefCell<World>>
}

impl WorldComponent {
    pub fn get<F: FnOnce(RefMut<World>)>(&self, closure: F) {
        if let Some(strong) = self.world.upgrade() {
            return closure(strong.borrow_mut());
        }

        fatal!("Weak<World> error");
    }
}

