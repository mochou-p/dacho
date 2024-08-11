// dacho/src/ecs/system.rs

// crates
use winit::{
    event::{MouseButton, WindowEvent},
    dpi::PhysicalPosition,
    keyboard::KeyCode
};

// super
use super::world::{State, World};

type         StateSystem = Box<dyn Fn    (&mut World, State, State)>;
type         StartSystem = Box<dyn FnOnce(&mut World)>;
type        UpdateSystem = Box<dyn Fn    (&mut World)>;
type      KeyboardSystem = Box<dyn Fn    (&mut World, KeyCode, bool)>;
type MousePositionSystem = Box<dyn Fn    (&mut World, PhysicalPosition<f64>)>;
type   MouseButtonSystem = Box<dyn Fn    (&mut World, MouseButton, bool)>;
type    MouseWheelSystem = Box<dyn Fn    (&mut World, f32, f32)>;
type         EventSystem = Box<dyn Fn    (&mut World, WindowEvent)>;

#[non_exhaustive]
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

