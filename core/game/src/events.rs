// dacho/core/game/src/events.rs

use {
    alloc::collections::LinkedList,
    core::time::Duration
};

use winit::{
    dpi::PhysicalPosition,
    event::{DeviceId, DeviceEvent, Modifiers, MouseButton, MouseScrollDelta}
};

use super::{Key, Time};


#[expect(clippy::exhaustive_enums, reason = "guaranteed to never change")]
#[derive(Debug)]
pub enum Event<GE> {
    Game(GE),
    Engine(EngineEvent)
}

#[non_exhaustive]
#[derive(Debug)]
pub enum EngineEvent {
    // --- engine flow ---
    Start,
    Update      { tick: usize },
    FixedUpdate { tick: usize },
    End,

    // --- input ---
    Device    { device_id: DeviceId, event:    DeviceEvent                         },
    Keyboard  { device_id: DeviceId, key:      Key, is_pressed: bool, repeat: bool },
    Modifiers { value:     Modifiers                                               },
    Mouse     { device_id: DeviceId, button:   MouseButton, is_pressed: bool       },
    Scroll    { device_id: DeviceId, delta:    MouseScrollDelta                    },
    Cursor    { device_id: DeviceId, position: PhysicalPosition<f64>               },

    // --- window ---
    CursorPresent { device_id: DeviceId, value: bool },
    Focused       {                      value: bool },
    Occluded      {                      value: bool }
}

#[non_exhaustive]
pub struct Events<GE> {
    pub queue: LinkedList<Node<GE>>
}

// not derive to not expect Default from GE
impl<GE> Default for Events<GE> {
    fn default() -> Self {
        Self { queue: LinkedList::new() }
    }
}

#[non_exhaustive]
pub struct Node<GE> {
    pub when:  f32,
    pub event: GE
}

impl<GE> Events<GE> {
    // todo: simplify
    #[expect(clippy::unwrap_used, reason = "temp")]
    pub fn do_after(&mut self, event: GE, delay: Duration, time: &Time) {
        let when = time.elapsed + delay.as_secs_f32();

        if self.queue.is_empty() {
            self.queue.push_front(Node { when, event });
            return;
        }

        let mut node = self.queue.cursor_front_mut();

        while when > node.current().unwrap().when {
            if node.peek_next().is_some() {
                node.move_next();
            } else {
                break;
            }
        }

        if when > node.current().unwrap().when {
            node.insert_after(Node { when, event });
        } else {
            node.insert_before(Node { when, event });
        }
    }
}
