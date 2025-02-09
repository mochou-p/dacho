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
            Game, Time
        },

        glam::{
            f32::{Vec2, Vec3, Vec4, Mat4, Quat},
            swizzles::{Vec2Swizzles, Vec3Swizzles, Vec4Swizzles},
            EulerRot
        },
        winit::{
            event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta},
            dpi::PhysicalPosition,
            keyboard::*,
            window::CursorGrabMode
        }
    };
}

