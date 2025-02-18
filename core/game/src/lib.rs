// dacho/core/game/src/lib.rs

#![feature(linked_list_cursors)]

extern crate alloc;

pub mod data;
pub mod events;

use {
    core::time::Duration,
    std::time::Instant
};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalPosition,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow},
    keyboard::{Key as LogicalKey, KeyLocation, PhysicalKey, SmolStr},
    window::WindowId
};

use {
    data::{commands::Command, Data},
    events::{Event, EngineEvent}
};

use {
    dacho_renderer::Renderer,
    dacho_window::Window
};


#[expect(clippy::exhaustive_structs, reason = "for now created by struct expr + ..default")]
pub struct Game<GD, GE> {
    pub title:        &'static str,
    pub clock:                 Clock,
    pub window:         Option<Window>,
    pub renderer:       Option<Renderer>,
    pub resumed:               bool,
    pub should_close:          bool, // temp
    pub fixed_update:   Option<f32>, // in seconds

    pub event_handler: fn(&mut Data<GD, GE>, &Event<GE>),
    pub data:          Data<GD, GE>
}

impl<GD, GE> Default for Game<GD, GE>
where
    GD: Default
{
    fn default() -> Self {
        Self {
            title:         "untitled game",
            clock:         Clock::default(),
            window:        None,
            renderer:      None,
            resumed:       false,
            should_close:  false,
            fixed_update:  None,
            event_handler: |_, _| {},
            data:          Data::<GD, GE>::default()
        }
    }
}

impl<GD, GE> Game<GD, GE> {
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

    // todo: dont play with a LL every frame,
    //       just cache the f32, cause most frames
    //       do not trigger any events
    #[expect(clippy::unwrap_used, reason = "temp")]
    fn check_events(&mut self) {
        while !self.data.engine.events.queue.is_empty() {
            {
                let node = self.data.engine.events.queue.front().unwrap();

                if node.when > self.data.engine.time.elapsed {
                    return;
                }
            }

            let node = self.data.engine.events.queue.pop_front().unwrap();
            (self.event_handler)(&mut self.data, &Event::Game(node.event));
        }
    }

    // todo: simplify
    #[expect(clippy::unwrap_used, reason = "temp")]
    fn check_fixed_update(&mut self) {
        let fixed_elapsed = self.clock.last_tick
            .duration_since(self.clock.fixed_last_tick)
            .as_secs_f32();

        if fixed_elapsed > self.fixed_update.unwrap() {
            let late = fixed_elapsed - self.fixed_update.unwrap();

            (self.event_handler)(&mut self.data, &Event::Engine(
                EngineEvent::FixedUpdate { tick: self.clock.fixed_logic_index }
            ));

            // temp
            let error_correction = Duration::from_secs_f32(
                if self.clock.fixed_logic_index == 0 {
                    late * 2.0
                } else {
                    late
                }
            );

            self.clock.fixed_logic_index += 1;
            self.clock.fixed_last_tick    = self.clock.last_tick - error_correction;
        }
    }

    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)
            .expect("failed to run the app in event loop");

        drop(self.renderer.expect("no Renderer"));
    }
}

impl<GD, GE> ApplicationHandler for Game<GD, GE> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.resumed {
            return;
        }

        self.resumed = true;

        self.window.get_or_insert(
            Window::new(self.title, 1600, 900, event_loop)
                .expect("failed to create Window")
        );

        #[expect(clippy::unwrap_used, reason = "Some in this context")]
        self.renderer.get_or_insert(
            Renderer::new(event_loop, self.window.as_ref().unwrap())
                .expect("failed to create Renderer")
        );

        (self.event_handler)(&mut self.data, &Event::Engine(
            EngineEvent::Start
        ));

        self.check_commands();

        self.clock = Clock::default();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.data.engine.time = Time {
            elapsed: self.clock.start    .elapsed().as_secs_f32(),
            delta:   self.clock.last_tick.elapsed().as_secs_f32()
        };

        self.clock.last_tick = Instant::now();

        (self.event_handler)(&mut self.data, &Event::Engine(
            EngineEvent::Update { tick: self.clock.logic_index }
        ));

        self.clock.logic_index += 1;

        if self.fixed_update.is_some() {
            self.check_fixed_update();
        }

        self.check_events();
        self.check_commands();

        if self.should_close {
            event_loop.exit();
            return;
        }

        self.renderer
            .as_ref()
            .expect("no Renderer")
            .wait_for_device();

        self.window
            .as_ref()
            .expect("no Window")
            .request_redraw();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        (self.event_handler)(&mut self.data, &Event::Engine(
            EngineEvent::End
        ));
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        device_id:    DeviceId,
        event:        DeviceEvent
    ) {
        (self.event_handler)(&mut self.data, &Event::Engine(
            EngineEvent::Device { device_id, event }
        ));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id:  WindowId,
        event:       WindowEvent
    ) {
        #[expect(clippy::wildcard_enum_match_arm, reason = "a lot of unused WindowEvents")]
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let meshes   = &mut self.data.engine.meshes;
                let renderer = self.renderer.as_mut()
                    .expect("no Renderer");

                if meshes.updated {
                    renderer.update_meshes(&meshes.data)
                        .expect("failed to update Meshes");

                    meshes.updated = false;
                }

                self.data.engine.camera.try_update();

                renderer.redraw(
                    self.data.engine.time.elapsed,
                    &self.data.engine.camera
                );
            },
            WindowEvent::KeyboardInput { device_id, event: key_event, is_synthetic } => {
                if is_synthetic { // todo: maybe hint code temperature
                    return;
                }

                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::Keyboard {
                        device_id,
                        key: Key {
                            physical: key_event.physical_key,
                            logical:  key_event.logical_key,
                            text:     key_event.text,
                            location: key_event.location
                        },
                        is_pressed: key_event.state.is_pressed(),
                        repeat:     key_event.repeat
                    }
                ));
            },
            WindowEvent::ModifiersChanged(value) => {
                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::Modifiers { value }
                ));
            },
            WindowEvent::MouseInput { device_id, state, button } => {
                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::Mouse {
                        device_id,
                        button,
                        is_pressed: state.is_pressed()
                    }
                ));
            },
            WindowEvent::MouseWheel { device_id, delta, .. } => {
                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::Scroll { device_id, delta }
                ));
            },
            WindowEvent::CursorMoved { device_id, position } => {
                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::Cursor { device_id, position }
                ));
            },
            WindowEvent::CursorEntered { device_id } => {
                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::CursorPresent { device_id, value: true }
                ));
            },
            WindowEvent::CursorLeft { device_id } => {
                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::CursorPresent { device_id, value: false }
                ));
            },
            WindowEvent::Focused(value) => {
                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::Focused { value }
                ));
            },
            WindowEvent::Occluded(value) => {
                (self.event_handler)(&mut self.data, &Event::Engine(
                    EngineEvent::Occluded { value }
                ));
            },
            _ => ()
        }
    }
}

pub struct Clock {
    start:             Instant,
    last_tick:         Instant,
    logic_index:       usize,
    fixed_last_tick:   Instant,
    fixed_logic_index: usize
}

impl Default for Clock {
    #[inline]
    #[must_use]
    fn default() -> Self {
        let now = Instant::now();

        Self {
            start:             now,
            last_tick:         now,
            logic_index:       0,
            fixed_last_tick:   now,
            fixed_logic_index: 0
        }
    }
}

#[non_exhaustive]
#[derive(Default)]
pub struct Time {
    pub elapsed: f32,
    pub delta:   f32
}

#[non_exhaustive]
#[derive(Debug)]
pub struct Key {
    pub physical: PhysicalKey,
    pub logical:  LogicalKey,
    pub text:     Option<SmolStr>,
    pub location: KeyLocation
}
