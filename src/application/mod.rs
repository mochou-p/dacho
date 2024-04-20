// dacho/src/application/mod.rs

mod camera;
mod scene;
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
    scene::Scene,
    window::Window
};

use super::renderer::Renderer;

pub struct Application {
    window:     Window,
    renderer:   Renderer,
    camera:     Camera,
    start_time: std::time::Instant
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        #[cfg(debug_assertions)]
        compile_shaders()?;

        let window     = Window::new("dacho", 1600, 900, event_loop)?;
        let scene      = Scene::demo()?;
        let renderer   = Renderer::new(event_loop, &window.window, window.width, window.height, &scene)?;
        let camera     = Camera::new(glam::Vec3::Y * 15.0);
        let start_time = std::time::Instant::now();

        Ok(Self { window, renderer, camera, start_time })
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
                self.renderer.redraw(self.camera.transform(), self.start_time.elapsed().as_secs_f32());
            },
            _ => ()
        }
    }
}

#[cfg(debug_assertions)]
pub fn compile_shaders() -> Result<()> {
    let mut filepath = std::env::current_dir()?;
    filepath.push("compile_shaders.py");

    std::process::Command::new("python")
        .arg(
            filepath
                .display()
                .to_string()
        )
        .spawn()?
        .wait_with_output()?;

    Ok(())
}

