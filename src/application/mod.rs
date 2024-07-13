// dacho/src/application/mod.rs

// modules
pub mod logger;
    mod timer;
pub mod window;

// crates
use {
    anyhow::Result,
    winit::{
        event::{Event, WindowEvent},
        event_loop::{EventLoop, EventLoopWindowTarget},
        keyboard::{KeyCode::Escape, PhysicalKey::Code}
    }
};

// mod
use {
    timer::Timer,
    window::Window
};

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
    timer:    Timer
}

impl Application {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        #[cfg(debug_assertions)] {
            log!(info, "Creating Application");
            log_indent!(true);

            block_on(compile_shaders())?;
        }

        let window   = Window::new("dacho", 1600, 900, event_loop)?;

        let timer = Timer::new(
            #[cfg(debug_assertions)]
            50
        );

        #[cfg(debug_assertions)]
        log_indent!(false);

        Ok(Self { window, timer })
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
            },
            Event::AboutToWait => {
                self.window.request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                #[cfg(debug_assertions)]
                self.timer.fps();
            },
            _ => ()
        }
    }
}

