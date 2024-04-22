// dacho/src/application/mod.rs

    mod camera;
#[cfg(debug_assertions)]
pub mod logger;
    mod scene;
    mod timer;
    mod window;

use anyhow::Result;

use glam::f32 as glam;

use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{KeyCode::*, PhysicalKey::Code}
};

#[cfg(debug_assertions)]
use logger::Logger;

use {
    camera::Camera,
    scene::Scene,
    timer::Timer,
    window::Window
};

use super::renderer::Renderer;

pub struct Application {
    window:   Window,
    renderer: Renderer,
    camera:   Camera,
    timer:    Timer
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        #[cfg(debug_assertions)] {
            Logger::info("Creating Application");
            Logger::indent(1);

            compile_shaders()?;
        }

        let window   = Window::new("dacho", 1600, 900, event_loop)?;
        let scene    = Scene::demo()?;
        let renderer = Renderer::new(event_loop, &window.window, window.width, window.height, &scene)?;
        let camera   = Camera::new(glam::Vec3::new(0.0, 60.0, 160.0));
        #[cfg(debug_assertions)]
        let timer    = Timer::new(50);
        #[cfg(not(debug_assertions))]
        let timer    = Timer::new();

        #[cfg(debug_assertions)]
        Logger::indent(-1);

        Ok(Self { window, renderer, camera, timer })
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

                self.camera.keyboard_input(event);
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                self.camera.mouse_motion(delta);
            },
            Event::AboutToWait => {
                self.renderer.wait_for_device();
                self.window.request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                self.renderer.redraw(self.camera.transform(), self.timer.elapsed());

                #[cfg(debug_assertions)]
                self.timer.fps();
            },
            _ => ()
        }
    }
}

#[cfg(debug_assertions)]
pub fn compile_shaders() -> Result<()> {
    Logger::info("Running shader compilation script");

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

