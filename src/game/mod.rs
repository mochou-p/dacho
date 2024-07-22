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
        application::ApplicationHandler,
        event::WindowEvent,
        event_loop::{ActiveEventLoop, EventLoop, ControlFlow::Poll},
        keyboard::{KeyCode::Escape, PhysicalKey::Code},
        window::WindowId
    }
};

// mod
use {
    timer::Timer,
    window::Window
};

// pub use
pub use winit::{keyboard::KeyCode, event::ElementState as KeyState};

// super
use super::ecs::world::{State, World};

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

        Self {
            title: String::from(title),
            world,
            timer,
            window: None,
            renderer: None
        }
    }

    #[inline]
    pub fn state(&mut self, default: State, state_system: impl Fn(&mut World, State, State) + 'static) {
        self.world.state_system = Some((default, Box::new(state_system)));
    }

    #[inline]
    pub fn start(&mut self, start_system: impl FnOnce(&mut World) + 'static) {
        self.world.start_systems.push(Box::new(start_system));
    }

    #[inline]
    pub fn update(&mut self, update_system: impl Fn(&mut World) + 'static) {
        self.world.update_systems.push(Box::new(update_system));
    }

    #[inline]
    pub fn keyboard(&mut self, keyboard_system: impl Fn(&mut World, KeyCode, KeyState) + 'static) {
        self.world.keyboard_systems.push(Box::new(keyboard_system));
    }

    #[tokio::main]
    #[allow(clippy::missing_panics_doc)]
    pub async fn run(mut self) {
        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        event_loop.set_control_flow(Poll);
        event_loop.run_app(&mut self);

        if let Some(renderer) = self.renderer {
            drop(renderer);
        }
    }
}

impl ApplicationHandler for Game {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.world.start();

        self.window = Some(
            Window::new(&self.title, 1600, 900, event_loop)
                .expect("failed to create a Window")
        );

        if let Some(window) = &self.window {
            self.renderer = Some(
                Renderer::new(
                    event_loop,
                    window,
                    &self.world.get_mesh_data()
                        .expect("failed to get world mesh data")
                ).expect("failed to create a Renderer")
            );
        } 
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.world.update();

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { event, is_synthetic, .. } => {
                if event.repeat {
                    return;
                }

                if event.physical_key == Code(Escape) {
                    self.window_event(event_loop, id, WindowEvent::CloseRequested);
                } else {
                    self.world.keyboard(&event);
                }
            },
            WindowEvent::RedrawRequested => {
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

