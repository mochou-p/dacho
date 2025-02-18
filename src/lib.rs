// dacho/src/lib.rs

#![allow(clippy::multiple_crate_versions, reason = "outside dacho's power")]

// a compile_warning would fit here if it existed,
// or if you could eprint from `const _: () = { ... }`
#[cfg(all(not(debug_assertions), feature = "vulkan-validation"))]
compile_error!("the `vulkan-validation` feature is for debugging only");

pub use {
    dacho_components as components,
    dacho_game       as game,

    glam as math,
    winit::{event, dpi, keyboard}
};

pub mod prelude {
    pub use {
        dacho_components::{Camera, Mesh},
        dacho_game::{
            data::{
                commands::Command,
                Data, EngineData
            },
            events::{Event, EngineEvent},
            Game, Key, Time
        },

        glam::{
            f32::{
                Vec2, Vec3, Vec3A, Vec4, Mat4, Quat,
                vec2, vec3, vec3a, vec4, mat4, quat
            },
            swizzles::{Vec2Swizzles, Vec3Swizzles, Vec4Swizzles},
            EulerRot
        },
        winit::{
            event::{DeviceEvent, ElementState, KeyEvent, MouseButton, MouseScrollDelta},
            dpi::PhysicalPosition,
            keyboard::{Key as LogicalKey, KeyCode, PhysicalKey},
            window::CursorGrabMode
        },

        super::default
    };
}

#[must_use]
pub fn default<T>() -> T
where
    T: Default
{
    T::default()
}
