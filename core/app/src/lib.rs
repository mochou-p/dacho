// dacho/core/app/src/lib.rs

extern crate alloc;

// modules
mod timer;

use alloc::rc::{Rc, Weak};
use core::{cell::{RefCell, RefMut}, mem::take};
use std::collections::HashMap;

// crates
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow::Poll},
    keyboard::{KeyCode::Escape, PhysicalKey::Code},
    window::WindowId
};

// mod
use timer::Timer;

// super
use dacho_components::Mesh;
use dacho_ecs::{entity::Entity, world::World, query::{Query, QueryFn, QueryTuple}};
use dacho_renderer::Renderer;
use dacho_log::{log, fatal, create_log};
use dacho_window::Window;

// pub use
pub use winit::{
    dpi::PhysicalPosition,
    event::{MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::KeyCode as Key
};

#[non_exhaustive]
pub enum Schedule {
    Start,
    Update
}

type System = Box<dyn FnMut(&[Rc<Entity>])>;

struct Systems {
    start:  Vec<System>,
    update: Vec<System>
}

#[non_exhaustive]
pub struct WorldComponent {
    world: Weak<RefCell<World>>
}

impl WorldComponent {
    pub fn get(&self, closure: impl FnOnce(RefMut<World>)) {
        if let Some(strong) = self.world.upgrade() {
            return closure(strong.borrow_mut());
        }

        fatal!("Weak<World> error");
    }
}

pub struct App {
    title:    String,
    world:    Rc<RefCell<World>>,
    systems:  Systems,
    timer:    Timer,
    window:   Option<Window>,
    renderer: Option<Renderer>
}

impl App {
    #[must_use]
    pub fn new(title: &str) -> Self {
        create_log!(info);

        let world = Rc::new(RefCell::new(World::new()));
        world.borrow_mut().spawn((WorldComponent { world: Rc::downgrade(&world) },));

        let systems = Systems { start: vec![], update: vec![] };

        let timer = Timer::new(
            #[cfg(debug_assertions)]
            50
        );

        Self {
            title:    String::from(title),
            world,
            systems,
            timer,
            window:   None,
            renderer: None
        }
    }

    fn get_schedule_systems(&mut self, schedule: Schedule) -> &mut Vec<System> {
        match schedule {
            Schedule::Start  => &mut self.systems.start,
            Schedule::Update => &mut self.systems.update
        }
    }

    pub fn add_system<T>(mut self, schedule: Schedule, mut system: impl QueryFn<T> + 'static) -> Self
    where
        T: QueryTuple
    {
        self.get_schedule_systems(schedule).push(
            Box::new(
                move |entities| {
                    if let Some(queries) = system.get_queries(entities) {
                        system.run(queries);
                    }
                }
            )
        );

        self
    }

    fn run_system_schedule(&mut self, schedule: Schedule) {
        for system in match schedule {
            Schedule::Start  => &mut self.systems.start,
            Schedule::Update => &mut self.systems.update
        } {
            let taken = take(&mut self.world.borrow_mut().entities);
            system(&taken);
            self.world.borrow_mut().entities.extend(taken);
        }
    }

    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        log!(info, "Running App");

        self.run_system_schedule(Schedule::Start);

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
            let mut meshes = vec![];

            let mut get_meshes = |(query,): (Query<(Mesh,)>,)| {
                let all_components = query.all();

                let mut map = HashMap::new();

                for components in &all_components {
                    let mesh = components.0.borrow();

                    map
                        .entry(mesh.id)
                        .or_insert(Vec::with_capacity(1))
                        .extend(mesh.model_matrix.to_cols_array());
                }

                meshes = map.into_iter().collect();
            };

            let mut meshess = |entities| {
                if let Some(queries) = get_meshes.get_queries(entities) {
                    get_meshes.run(queries);
                }
            };

            meshess(&self.world.borrow().entities);

            self.renderer = Some(
                Renderer::new(
                    event_loop,
                    window,
                    meshes
                ).expect("failed to create a Renderer")
            );
        }

        log!(info, "<<< dacho is initialized >>>");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.run_system_schedule(Schedule::Update);

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

                if event.physical_key == Code(Escape) {
                    self.window_event(
                        event_loop,
                        window_id,
                        WindowEvent::CloseRequested
                    );
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

