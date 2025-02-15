// dacho/core/game/src/lib.rs

#![expect(internal_features, reason = "chill down rare synthetic key events")]
#![feature(core_intrinsics, linked_list_cursors)]

extern crate alloc;

pub mod data;
pub mod events;

use std::time::Instant;

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

    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self)
            .expect("failed to run the app in event loop");

        drop(self.renderer.expect("renderer is None"));
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

        self.renderer.get_or_insert(
            Renderer::new(event_loop, self.window.as_ref().expect("window is None"))
                .expect("failed to create Renderer")
        );

	(self.event_handler)(&mut self.data, &Event::Engine(
	    EngineEvent::Start
	));

        self.check_commands();

        self.clock = Clock::default();
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
	self.check_commands();

        if self.should_close {
            event_loop.exit();
            return;
        }

        self.data.engine.time = Time {
            elapsed: self.clock.start    .elapsed().as_secs_f32(),
            delta:   self.clock.last_tick.elapsed().as_secs_f32()
        };

        self.clock.last_tick = Instant::now();

	(self.event_handler)(&mut self.data, &Event::Engine(
	    EngineEvent::Update
	));

	self.check_events();

        self.renderer
            .as_ref()
            .expect("window is None")
            .wait_for_device();

        self.window
            .as_ref()
            .expect("window is None")
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
			.expect("failed to update meshes");

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
