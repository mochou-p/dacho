// dacho/src/application/mod.rs

mod window;

use anyhow::Result;

use glam::f32 as glam;

use winit::{
    event::{DeviceEvent, Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{KeyCode::*, PhysicalKey::Code}
};

use window::Window;

use super::renderer::Renderer;

pub struct Application {
    window:     Window,
    renderer:   Renderer,
    position:   glam::Vec3,
    movement:   MovementVector,
    direction:  glam::Vec3
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let window     =  Window::new("dacho", 1600, 900, event_loop)?;
        let renderer   =  Renderer::new(event_loop, &window.window, window.width, window.height)?;
        let position   =  glam::Vec3::Y * 15.0;
        let movement   =  ((0.0, 0.0), (0.0, 0.0), (0.0, 0.0));
        let direction  = -glam::Vec3::Z;

        Ok(
            Self {
                window,
                renderer,
                position,
                movement,
                direction
            }
        )
    }

    pub fn handle_event<T>(&mut self, event: &Event<T>, elwt: &EventLoopWindowTarget<T>) {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event, is_synthetic: false, .. }, .. } => {
                if event.physical_key == Code(Escape) {
                    elwt.exit();
                }

                self.keyboard_input(&event);
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                self.mouse_input(&delta);
            },
            Event::AboutToWait => {
                self.renderer.wait_for_device();
                self.update();
                self.window.request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                self.renderer.redraw(&self.position, &self.direction);
            },
            _ => ()
        }
    }

    fn keyboard_input(&mut self, event: &KeyEvent) {
        if event.repeat {
            return;
        }

        static SPEED: f32 = 0.2;

        match event.physical_key {
            Code(KeyA)      => { self.movement.0.0 = -SPEED * (1.0 - event.state as i32 as f32); },
            Code(KeyD)      => { self.movement.0.1 =  SPEED * (1.0 - event.state as i32 as f32); },
            Code(KeyW)      => { self.movement.2.0 = -SPEED * (1.0 - event.state as i32 as f32); },
            Code(KeyS)      => { self.movement.2.1 =  SPEED * (1.0 - event.state as i32 as f32); },
            Code(ShiftLeft) => { self.movement.1.0 = -SPEED * (1.0 - event.state as i32 as f32); },
            Code(Space)     => { self.movement.1.1 =  SPEED * (1.0 - event.state as i32 as f32); },
            _ => ()
        }
    }

    fn mouse_input(&mut self, delta: &(f64, f64)) {
        static SPEED:   f32 = -0.001;
        static PHI_MIN: f32 = -std::f32::consts::PI * 0.5 + f32::EPSILON;
        static PHI_MAX: f32 =  std::f32::consts::PI * 0.5 - f32::EPSILON;

        unsafe {
            static mut THETA: f32 = std::f32::consts::PI;
            static mut PHI:   f32 = 0.0;

            THETA += delta.0 as f32 * SPEED;

            PHI = (PHI + delta.1 as f32 * SPEED).clamp(PHI_MIN, PHI_MAX);

            self.direction.x = THETA.sin() * PHI.cos();
            self.direction.y = PHI.sin();
            self.direction.z = THETA.cos() * PHI.cos();
        }
    }

    fn update(&mut self) {
        self.position += movement_to_vec3(&self.movement);
    }
}

type MovementVector = ((f32, f32), (f32, f32), (f32, f32));

fn movement_to_vec3(m: &MovementVector) -> glam::Vec3 {
    glam::Vec3::new(
        m.0.0 + m.0.1,
        m.1.0 + m.1.1,
        m.2.0 + m.2.1
    )
}

