// dacho/core/app/src/lib.rs

extern crate alloc;

mod timer;

use {
    core::{any::TypeId, pin::Pin},
    alloc::collections::BTreeSet
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow::Poll},
    keyboard::PhysicalKey,
    window::WindowId
};

use timer::Timer;

use {
    dacho_ecs::{query::QueryTuple, world::World},
    dacho_renderer::Renderer,
    dacho_log::{fatal, log, create_log},
    dacho_window::Window
};

pub use winit::keyboard::KeyCode;


type System = (Box<dyn Fn()>, usize, Vec<BTreeSet<TypeId>>);

pub struct App {
        title:    String,
    pub world:    Pin<Box<World>>,
        systems:  Vec<System>,
        timer:    Timer,
        window:   Option<Window>,
        renderer: Option<Renderer>
}

impl App {
    #[must_use]
    pub fn new(title: &str) -> Self {
        create_log!(info);

        let world = Box::pin(World::new());
        let timer = Timer::new();

        Self {
            title:    String::from(title),
            world,
            systems:  vec![],
            timer,
            window:   None,
            renderer: None
        }
    }

    pub fn insert<T, F>(&mut self, func: F)
    where
        T: QueryTuple + 'static,
        F: Fn(&T)     + 'static
    {
        let query_tuple = T::new(self.world_mut_ptr());

        self.systems.push((Box::new(move || func(&query_tuple)), 0, T::get_all_sets()));
    }

    fn world_mut_ptr(&mut self) -> *mut World {
        Pin::<&mut _>::into_inner(self.world.as_mut()) as *mut _
    }

    fn setup(&mut self) {
        let entity_sets = self.world.entities.keys().cloned().collect::<Vec<_>>();
        let mut x;

        for (_, match_count, query_sets) in &mut self.systems {
            for query_set in query_sets {
                x = false;

                for entity_set in &entity_sets {
                    if query_set.is_subset(entity_set) {
                        self.world.query_matches
                            .entry(query_set.clone())
                            // not ideal
                            .and_modify(|vec| if !vec.contains(entity_set) {
                                vec.push(entity_set.clone());
                            })
                            .or_insert(vec![entity_set.clone()]);

                        x = true;
                    }
                }

                if x {
                    *match_count += 1;
                }
            }
        }
    }

    fn run_systems(&mut self) {
        for (system, satisfied, sets) in &self.systems {
            if *satisfied == sets.len() {
                system();
            }
        }
    }

    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        log!(info, "Running App");

        if self.systems.is_empty() {
            fatal!("App has no Systems, nothing to do");
        }

        self.setup();

        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        event_loop.set_control_flow(Poll);
        event_loop.run_app(&mut self).expect("failed to run the app in event loop");

        log!(info, "<<< dacho is shutting down >>>");

        if let Some(renderer) = self.renderer {
            drop(renderer);
        }

        log!(info, "<<< graceful shutdown >>>");
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        log!(debug, "Resuming ActiveEventLoop");

        if self.window.is_some() {
            return;
        }

        self.window = Some(
            Window::new(&self.title, 1600, 900, event_loop)
                .expect("failed to create a Window")
        );

        if let Some(window) = &self.window {
            self.renderer = Some(
                Renderer::new(
                    event_loop,
                    window,
                    vec![]
                ).expect("failed to create a Renderer")
            );
        }

        log!(info, "<<< dacho is initialized >>>");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.run_systems();

        if let Some(renderer) = &mut self.renderer {
            renderer.wait_for_device();
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    #[expect(clippy::only_used_in_recursion,  reason = "WindowId, to trigger CloseRequested on [Escape]")]
    #[expect(clippy::renamed_function_params, reason = "winit reuses `event`")]
    fn window_event(
        &mut self,
        event_loop:   &ActiveEventLoop,
        window_id:     WindowId,
        window_event:  WindowEvent
    ) {
        #[expect(clippy::wildcard_enum_match_arm, reason = "lots of unused winit events")]
        match window_event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { event, is_synthetic, .. } => {
                if event.repeat || is_synthetic {
                    return;
                }

                if let PhysicalKey::Code(code) = event.physical_key {
                    if code == KeyCode::Escape {
                        self.window_event(
                            event_loop,
                            window_id,
                            WindowEvent::CloseRequested
                        );
                    }
                }
            },
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.redraw(self.timer.elapsed());
                };

                event_loop.exit();
            },
            _ => ()
        }
    }
}

