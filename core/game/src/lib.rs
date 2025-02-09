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
    event::{ElementState, DeviceEvent, DeviceId, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
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
#[expect(clippy::exhaustive_structs, reason = "for now created by struct expr + ..default")]
#[expect(clippy::type_complexity,    reason = "will clean up later")]
pub struct Game<GD: 'static> {
    pub title:    &'static str,
    pub clock:             Clock,
    pub window:     Option<Window>,
    pub renderer:   Option<Renderer>,
    pub should_close:      bool, // temp

    pub    start_systems: &'static [fn(&mut Data<GD>                            )],
    pub   update_systems: &'static [fn(&mut Data<GD>, &Time                     )],
    pub keyboard_systems: &'static [fn(&mut Data<GD>, &KeyEvent                 )],
    pub    mouse_systems: &'static [fn(&mut Data<GD>,  MouseButton, ElementState)],
    pub   cursor_systems: &'static [fn(&mut Data<GD>, &PhysicalPosition<f64>    )],
    pub   motion_systems: &'static [fn(&mut Data<GD>, &(f64, f64)               )],
    pub   scroll_systems: &'static [fn(&mut Data<GD>, &MouseScrollDelta         )],
    pub    focus_systems: &'static [fn(&mut Data<GD>,  bool                     )],
    pub      end_systems: &'static [fn(&mut Data<GD>                            )],

    pub data: Data<GD>
}

impl<GD> Game<GD> {
    fn check_commands(&mut self) {
        for command in self.data.engine.commands.queue.drain(..) {
            match command {
                Command::Exit => {
                    self.should_close = true;
                },

                Command::SetCursorGrab(mode) => {
                    let window = self.window.as_mut().expect("no Window");
                    window.raw.set_cursor_grab(mode)
                        .expect("failed to set window CursorGrabMode");
                },
                Command::SetCursorPosition((x, y)) => {
                    let window = self.window.as_mut().expect("no Window");
                    window.raw.set_cursor_position(PhysicalPosition { x, y })
                        .expect("failed to set window CursorPosition");
                },
                Command::SetCursorVisible(value) => {
                    let window = self.window.as_mut().expect("no Window");
                    window.raw.set_cursor_visible(value);
                }
            }
        }
    }

    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        for system in self.start_systems {
            system(&mut self.data);
            // todo: move the command check from resumed here
            //       when renderer and window are not options
            // (or maybe start systems run before window intentionally)
        }

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)
            .expect("failed to run the app in event loop");

        drop(self.renderer.expect("renderer is None"));
    }
}

impl<GD> ApplicationHandler for Game<GD> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window.get_or_insert(
            Window::new(self.title, 1600, 900, event_loop)
                .expect("failed to create Window")
        );

        self.renderer.get_or_insert(
            Renderer::new(event_loop, self.window.as_ref().expect("window is None"))
                .expect("failed to create Renderer")
        );

        self.check_commands();

        self.clock = Clock::default();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.should_close {
            event_loop.exit();
            return;
        }

        let time = Time {
            elapsed: self.clock.start    .elapsed().as_secs_f32(),
            delta:   self.clock.last_tick.elapsed().as_secs_f32()
        };

        self.clock.last_tick = Instant::now();

        for system in self.update_systems {
            system(&mut self.data, &time);
            self.check_commands();
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

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id:   DeviceId,
        event:        DeviceEvent
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            for system in self.motion_systems {
                system(&mut self.data, &delta);
                self.check_commands();
            }
        }
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
                        self.check_commands();
                    }
                }
            },
            WindowEvent::MouseInput { button, state, .. } => {
                for system in self.mouse_systems {
                    system(&mut self.data, button, state);
                    self.check_commands();
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                for system in self.scroll_systems {
                    system(&mut self.data, &delta);
                    self.check_commands();
                }
            },
            WindowEvent::Focused(value) => {
                for system in self.focus_systems {
                    system(&mut self.data, value);
                    self.check_commands();
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                for system in self.cursor_systems {
                    system(&mut self.data, &position);
                    self.check_commands();
                }
            },
            _ => ()
        }
    }

    #[inline]
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        for system in self.end_systems {
            system(&mut self.data);
            self.check_commands();
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

