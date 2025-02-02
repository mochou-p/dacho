// dacho/core/game/src/lib.rs

#![expect(internal_features, reason = "chill down rare synthetic key events")]
#![feature(core_intrinsics)]

use {
    core::intrinsics::cold_path,
    std::{collections::HashMap, time::Instant}
};

use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    dpi::PhysicalPosition,
    window::WindowId
};

use {
    dacho_components::{Camera, Mesh},
    dacho_renderer::Renderer,
    dacho_window::Window
};


#[non_exhaustive]
pub enum Command {
    Exit,
    Noop
}

#[derive(Default)]
pub struct Commands {
    queue: Vec<Command>
}

impl Commands {
    #[inline]
    pub fn submit(&mut self, command: Command) {
        self.queue.push(command);
    }

    #[inline]
    pub fn submit_all<C>(&mut self, commands: C)
    where
        C: IntoIterator<Item = Command>
    {
        self.queue.extend(commands);
    }
}

#[derive(Default)]
pub struct Meshes {
    updated: bool,
    data:    HashMap<u32, Vec<f32>>
}

impl Meshes {
    pub fn push(&mut self, mesh: Mesh) {
        self.data
            .entry(mesh.id)
            .and_modify(|vec| vec.extend(
                mesh.model_matrix
                    .to_cols_array()
            ))
            .or_insert(
                mesh.model_matrix
                    .to_cols_array()
                    .into()
            );

        self.updated = true;
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

pub trait World {
    fn get_mut_commands(&mut self) -> &mut Commands;
    fn get_mut_meshes  (&mut self) -> &mut Meshes;
    fn get_mut_camera  (&mut self) -> &mut Camera;
}

#[derive(Default)]
#[expect(clippy::exhaustive_structs, reason = "created by struct expr + ..default")]
pub struct Game<W: 'static>
where
    W: World
{
    pub title:    &'static str,
    pub clock:             Clock,
    pub window:     Option<Window>,
    pub renderer:   Option<Renderer>,

    pub    start_systems: &'static [fn(&mut W                            )],
    pub   update_systems: &'static [fn(&mut W, &Time                     )],
    pub keyboard_systems: &'static [fn(&mut W, &KeyEvent                 )],
    pub    mouse_systems: &'static [fn(&mut W,  MouseButton, ElementState)],
    pub   cursor_systems: &'static [fn(&mut W, &PhysicalPosition<f64>    )],
    pub   scroll_systems: &'static [fn(&mut W, &MouseScrollDelta         )],
    pub    focus_systems: &'static [fn(&mut W,  bool                     )],
    pub      end_systems: &'static [fn(                                  )],

    pub world: W
}

impl<W> Game<W>
where
    W: World
{
    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        for system in self.start_systems {
            system(&mut self.world);
        }

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)
            .expect("failed to run the app in event loop");

        for system in self.end_systems {
            system();
        }

        drop(self.renderer.expect("renderer is None"));
    }
}

impl<W> ApplicationHandler for Game<W>
where
    W: World
{
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
        for command in self.world.get_mut_commands().queue.drain(..) {
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
            system(&mut self.world, &time);
        }

        self.world.get_mut_camera().try_update();

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
                let meshes   = self.world.get_mut_meshes();
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
                    self.world.get_mut_camera()
                );
            },
            WindowEvent::KeyboardInput { event, is_synthetic, .. } => {
                if is_synthetic {
                    cold_path();
                } else {
                    for system in self.keyboard_systems {
                        system(&mut self.world, &event);
                    }
                }
            },
            WindowEvent::MouseInput { button, state, .. } => {
                for system in self.mouse_systems {
                    system(&mut self.world, button, state);
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                for system in self.scroll_systems {
                    system(&mut self.world, &delta);
                }
            },
            WindowEvent::Focused(value) => {
                for system in self.focus_systems {
                    system(&mut self.world, value);
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                for system in self.cursor_systems {
                    system(&mut self.world, &position);
                }
            },
            _ => ()
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

