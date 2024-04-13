// dacho/src/application/mod.rs

mod camera;
mod window;

use anyhow::Result;

use glam::f32 as glam;

use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{KeyCode::*, PhysicalKey::Code}
};

use {
    camera::Camera,
    window::Window
};

use super::renderer::Renderer;

pub struct Application {
    window:    Window,
    renderer:  Renderer,
    camera:    Camera
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        let window   = Window::new("dacho", 1600, 900, event_loop)?;
        let renderer = Renderer::new(event_loop, &window.window, window.width, window.height)?;
        let camera   = Camera::new(glam::Vec3::Y * 15.0);

        Ok(
            Self {
                window,
                renderer,
                camera
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

                self.camera.keyboard_input(&event);
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                self.camera.mouse_motion(&delta);
            },
            Event::AboutToWait => {
                self.renderer.wait_for_device();
                self.window.request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                self.renderer.redraw(self.camera.transform());
            },
            _ => ()
        }
    }
}

