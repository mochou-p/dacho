// dacho/core/game/src/lib.rs

#![expect(internal_features, reason = "chill down rare synthetic key events")]
#![feature(core_intrinsics)]

pub mod data;

use {
    core::intrinsics::cold_path,
    std::time::Instant
};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    dpi::PhysicalPosition,
    window::WindowId
};

use data::{commands::Command, Data};

use {
    dacho_renderer::Renderer,
    dacho_window::Window
};


#[derive(Default)]
#[expect(clippy::exhaustive_structs, reason = "created by struct expr + ..default")]
#[expect(clippy::type_complexity,    reason = "will clean up later"               )]
pub struct Game<GD: 'static> {
    pub title:    &'static str,
    pub clock:             Clock,
    pub window:     Option<Window>,
    pub renderer:   Option<Renderer>,

    pub    start_systems: &'static [fn(&mut Data<GD>                            )],
    pub   update_systems: &'static [fn(&mut Data<GD>, &Time                     )],
    pub keyboard_systems: &'static [fn(&mut Data<GD>, &KeyEvent                 )],
    pub    mouse_systems: &'static [fn(&mut Data<GD>,  MouseButton, ElementState)],
    pub   cursor_systems: &'static [fn(&mut Data<GD>, &PhysicalPosition<f64>    )],
    pub   scroll_systems: &'static [fn(&mut Data<GD>, &MouseScrollDelta         )],
    pub    focus_systems: &'static [fn(&mut Data<GD>,  bool                     )],
    pub      end_systems: &'static [fn(&mut Data<GD>                            )],

    pub data: Data<GD>
}

impl<GD> Game<GD> {
    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        for system in self.start_systems {
            system(&mut self.data);
        }

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)
            .expect("failed to run the app in event loop");

        drop(self.renderer.expect("renderer is None"));
    }
}

impl<D> ApplicationHandler for Game<D> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window.get_or_insert(
            Window::new(self.title, 1600, 900, event_loop)
                .expect("failed to create Window")
        );

        self.renderer.get_or_insert(
            Renderer::new(event_loop, self.window.as_ref().expect("window is None"))
                .expect("failed to create Renderer")
        );

        self.clock = Clock::default();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        for command in self.data.engine.commands.queue.drain(..) {
            if let Command::Exit = command {
                return event_loop.exit();
            }
        }

        let time = Time {
            elapsed: self.clock.start    .elapsed().as_secs_f32(),
            delta:   self.clock.last_tick.elapsed().as_secs_f32()
        };

        self.clock.last_tick = Instant::now();

        for system in self.update_systems {
            system(&mut self.data, &time);
        }

        self.data.engine.camera.try_update();

        self.renderer
            .as_ref()
            .expect("window is None")
            .wait_for_device();

        self.window
            .as_ref()
            .expect("window is None")
            .request_redraw();
    }

    #[expect(clippy::renamed_function_params, reason = "winit reuses `event`")]
    fn window_event(
        &mut self,
        event_loop:   &ActiveEventLoop,
        _window_id:    WindowId,
        window_event:  WindowEvent
    ) {
        #[expect(clippy::wildcard_enum_match_arm, reason = "lots of unused winit events")]
        match window_event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let meshes   = &mut self.data.engine.meshes;
                let renderer = self.renderer
                    .as_mut()
                    .expect("renderer is None");

                if meshes.updated {
                    renderer.update_meshes(&meshes.data)
                        .expect("failed to update meshes");

                    meshes.updated = false;
                }

                renderer.redraw(
                    self.clock.start.elapsed().as_secs_f32(),
                    &self.data.engine.camera
                );
            },
            WindowEvent::KeyboardInput { event, is_synthetic, .. } => {
                if is_synthetic {
                    cold_path();
                } else {
                    for system in self.keyboard_systems {
                        system(&mut self.data, &event);
                    }
                }
            },
            WindowEvent::MouseInput { button, state, .. } => {
                for system in self.mouse_systems {
                    system(&mut self.data, button, state);
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                for system in self.scroll_systems {
                    system(&mut self.data, &delta);
                }
            },
            WindowEvent::Focused(value) => {
                for system in self.focus_systems {
                    system(&mut self.data, value);
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                for system in self.cursor_systems {
                    system(&mut self.data, &position);
                }
            },
            _ => ()
        }
    }

    #[inline]
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        for system in self.end_systems {
            system(&mut self.data);
        }
    }
}

pub struct Clock {
    start:     Instant,
    last_tick: Instant
}

impl Default for Clock {
    #[inline]
    #[must_use]
    fn default() -> Self {
        let now = Instant::now();

        Self {
            start:     now,
            last_tick: now
        }
    }
}

#[non_exhaustive]
pub struct Time {
    pub elapsed: f32,
    pub delta:   f32
}

