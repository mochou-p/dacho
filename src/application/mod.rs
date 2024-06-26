// dacho/src/application/mod.rs

// modules
    mod camera;
pub mod logger;
pub mod scene;
    mod timer;
    mod window;

// crates
use {
    anyhow::Result,
    glam::f32 as glam,
    winit::{
        event::{DeviceEvent, Event, WindowEvent},
        event_loop::{EventLoop, EventLoopWindowTarget},
        keyboard::{KeyCode::Escape, PhysicalKey::Code}
    }
};

// mod
use {
    camera::Camera,
    scene::Data,
    timer::Timer,
    window::Window
};

// super
use super::renderer::Renderer;

// debug
#[cfg(debug_assertions)]
use {
    futures::executor::block_on,
    logger::Logger,
    super::log_indent,
    crate::{
        shader::compile_shaders,
        log
    }
};

pub struct Application {
    window:   Window,
    renderer: Renderer,
    camera:   Camera,
    timer:    Timer
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>, data: &Data) -> Result<Self> {
        #[cfg(debug_assertions)] {
            log!(info, "Creating Application");
            log_indent!(true);

            block_on(compile_shaders())?;
        }

        let window = Window::new("dacho", 1600, 900, event_loop)?;

        let renderer = Renderer::new(
            event_loop, window.raw(), window.width, window.height, data
        )?;

        let camera = Camera::new(glam::Vec3::new(0.0, -1.0, 13.0));
        let timer  = Timer::new(
            #[cfg(debug_assertions)]
            50
        );

        #[cfg(debug_assertions)]
        log_indent!(false);

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

