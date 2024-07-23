// dacho/src/ecs/system.rs

// crates
use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    dpi::PhysicalPosition,
    keyboard::KeyCode
};

// super
use super::world::{State, World};

pub type         StateSystem = Box<dyn Fn    (&mut World, State, State)>;
pub type         StartSystem = Box<dyn FnOnce(&mut World)>;
pub type        UpdateSystem = Box<dyn Fn    (&mut World)>;
pub type      KeyboardSystem = Box<dyn Fn    (&mut World, KeyCode, ElementState)>;
pub type MousePositionSystem = Box<dyn Fn    (&mut World, PhysicalPosition<f64>)>;
pub type   MouseButtonSystem = Box<dyn Fn    (&mut World, MouseButton, ElementState)>;
pub type    MouseWheelSystem = Box<dyn Fn    (&mut World, f32, f32)>;
pub type         EventSystem = Box<dyn Fn    (&mut World, WindowEvent)>;

pub struct Systems {
    pub state:          Option<(State, StateSystem)>,
    pub start:          Vec<StartSystem>,
    pub update:         Vec<UpdateSystem>,
    pub keyboard:       Vec<KeyboardSystem>,
    pub mouse_position: Vec<MousePositionSystem>,
    pub mouse_button:   Vec<MouseButtonSystem>,
    pub mouse_wheel:    Vec<MouseWheelSystem>,
    pub event:          Vec<EventSystem>
}

impl Systems {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            state:          None,
            start:          vec![],
            update:         vec![],
            keyboard:       vec![],
            mouse_position: vec![],
            mouse_button:   vec![],
            mouse_wheel:    vec![],
            event:          vec![]
        }
    }
}

