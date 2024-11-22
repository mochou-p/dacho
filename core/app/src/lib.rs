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
    dacho_ecs::{query::QueryTuple, world::{World, WorldComponent}},
    dacho_renderer::Renderer,
    dacho_log::{log, create_log},
    dacho_window::Window
};

pub use winit::keyboard::KeyCode;


type System = (Box<dyn Fn()>, Vec<(BTreeSet<TypeId>, u32)>);

pub struct App {
        title:     String,
    pub world:     Pin<Box<World>>,
        systems:   Vec<System>,
        timer:     Timer,
        window:    Option<Window>,
        renderer:  Option<Renderer>,
        no_window: usize
}

impl App {
    #[must_use]
    pub fn new(title: &str) -> Self {
        create_log!(info);

        let world = Box::pin(World::new());
        let timer = Timer::new();

        Self {
            title:     String::from(title),
            world,
            systems:   vec![],
            timer,
            window:    None,
            renderer:  None,
            no_window: 0
        }
    }

    pub fn no_window_run_n_times(&mut self, count: usize) {
        self.no_window = count;
    }

    pub fn insert<T, F>(&mut self, func: F)
    where
        T: QueryTuple + 'static,
        F: Fn(&T)     + 'static
    {
        let query_tuple = T::new(self.world_mut_ptr());

        self.systems.push((Box::new(move || func(&query_tuple)), T::get_all_sets()));
    }

    fn world_mut_ptr(&mut self) -> *mut World {
        Pin::<&mut _>::into_inner(self.world.as_mut()) as *mut _
    }

    fn setup(&mut self) {
        let entity_sets = self.world.entities.keys().cloned().collect::<Vec<_>>();

        for (_, queries) in &mut self.systems {
            for (query, matches) in queries {
                for entity_set in &entity_sets {
                    if query.is_subset(entity_set) {
                        self.world.query_matches
                            .entry(query.clone())
                            // not ideal
                            .and_modify(|vec| if !vec.contains(entity_set) {
                                vec.push(entity_set.clone());
                            })
                            .or_insert(vec![entity_set.clone()]);

                        *matches += 1;
                    }
                }
            }
        }
    }

    fn run_systems(&mut self) {
        for (system, queries) in &self.systems {
            if queries.iter().all(|x| x.1 != 0) {
                system();
            }
        }

        self.world.check_moves();

        if self.world.changed_sets.0.len() + self.world.changed_sets.1.len() != 0 {
            for (_, queries) in &mut self.systems {
                for (query, matches) in queries {
                    for new_set in &self.world.changed_sets.0 {
                        if query.is_subset(new_set) {
                            *matches += 1;
                        }
                    }

                    for removed_set in &self.world.changed_sets.1 {
                        if query.is_subset(removed_set) {
                            *matches -= 1;
                        }
                    }
                }
            }

            self.world.changed_sets.0.clear();
            self.world.changed_sets.1.clear();
        }
    }

    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        log!(info, "Running App");

        // changes global clippy::pedantic from forbid to deny
        #[expect(clippy::panic, reason = "not dacho_log::fatal because tests cannot capture exit, only panic")]
        if self.systems.is_empty() {
            panic!("App has no Systems, nothing to do");
        }

        // SAFETY: raw pointer
        let world_c = unsafe { WorldComponent::new(self.world_mut_ptr()) };
        self.world.spawn((world_c,));

        self.setup();

        if self.no_window != 0 {
            for _ in 0..self.no_window {
                self.run_systems();
            }

            return;
        }

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
            },
            _ => ()
        }
    }
}

