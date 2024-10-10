// dacho/core/components/input/src/lib.rs

use winit::{event::MouseButton, keyboard::KeyCode};


#[expect(clippy::exhaustive_structs, reason = "reexported, but created by struct expression")]
pub struct KeyComponent {
    pub code: KeyCode,
    pub down: bool
}

#[expect(clippy::exhaustive_structs, reason = "reexported, but created by struct expression")]
pub struct MousePositionComponent {
    pub x: f64,
    pub y: f64
}

#[expect(clippy::exhaustive_structs, reason = "reexported, but created by struct expression")]
pub struct MouseButtonComponent {
    pub button: MouseButton,
    pub down:   bool
}

#[expect(clippy::exhaustive_structs, reason = "reexported, but created by struct expression")]
pub struct MouseWheelComponent {
    pub x: f32,
    pub y: f32
}

