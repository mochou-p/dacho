// dacho/src/game/mod.rs

// modules
pub mod logger;
    mod timer;
pub mod window;

// std
use std::mem::take;

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

// super
use super::ecs::{
    system::UpdateSystem,
    world::World
};

// crate
use crate::renderer::Renderer;

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

struct Systems {
    update: Vec<UpdateSystem>
}

pub struct Game {
    title:    String,
    world:    World,
    timer:    Timer,
    window:   Option<Window>,
    renderer: Option<Renderer>
}

impl Game {
    #[must_use]
    pub fn new(title: &str) -> Self {
        let world = World::new();

        let timer = Timer::new(
            #[cfg(debug_assertions)]
            50
        );

        Self { title: String::from(title), world, timer, window: None, renderer: None }
    }

    pub fn start(&mut self, callback: impl FnOnce(&mut World) + 'static) {
        self.world.start_systems.push(Box::new(callback));
    }

    pub fn update(&mut self, callback: impl Fn(&mut World) + 'static) {
        self.world.update_systems.push(Box::new(callback));
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn run(mut self) {
        self.world.start();

        let event_loop = winit::event_loop::EventLoop::new()
            .expect("failed to create an EventLoop");

        self.window = Some(
            Window::new(&self.title, 1600, 900, &event_loop)
                .expect("failed to create a Window")
        );

        if let Some(window) = &self.window {
            self.renderer = Some(
                Renderer::new(&event_loop, window)
                    .expect("failed to create a Renderer")
            );
        }

        event_loop.run(move |event, elwt| {
            self.handle_event(&event, elwt);
        }).expect("failed to run an EventLoop");
    }

    fn handle_event<T>(&mut self, event: &Event<T>, elwt: &EventLoopWindowTarget<T>) {
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
                self.world.update();

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.redraw(self.timer.elapsed());
                };

                #[cfg(debug_assertions)]
                self.timer.fps();
            },
            _ => ()
        }
    }
}

