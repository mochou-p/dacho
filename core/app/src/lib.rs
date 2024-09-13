// dacho/core/app/src/lib.rs

// modules
mod timer;

use core::{cell::RefCell, mem::take};
use std::rc::{Rc, Weak};

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
use dacho_ecs::{entity::Entity, world::World, query::{QueryFn, QueryTuple}};
use dacho_renderer::Renderer;
use dacho_log::{log, fatal, create_log};
use dacho_window::Window;

// pub use
pub use winit::{
    dpi::PhysicalPosition,
    event::{MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::KeyCode as Key
};

type System = Box<dyn Fn(&Vec<Rc<Entity>>)>;

#[non_exhaustive]
pub struct WorldComponent {
    world: Weak<RefCell<World>>
}

impl WorldComponent {
    pub fn get(&self) -> Rc<RefCell<World>> {
        if let Some(strong) = self.world.upgrade() {
            return strong;
        }

        fatal!("could not get World");
    }
}

pub struct App {
    title:    String,
    world:    Rc<RefCell<World>>,
    systems:  Vec<System>,
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

        let timer = Timer::new(
            #[cfg(debug_assertions)]
            50
        );

        Self {
            title:    String::from(title),
            world,
            systems:  vec![],
            timer,
            window:   None,
            renderer: None
        }
    }

    pub fn add_system<T>(&mut self, system: impl QueryFn<T> + 'static)
    where
        T: QueryTuple
    {
        self.systems.push(Box::new(move |entities| {
            if let Some(queries) = system.get_queries(entities) {
                system.call(queries);
            }
        }));
    }

    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        log!(info, "Running App");

        for system in &self.systems {
            // TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP
            let taken = take(&mut self.world.borrow_mut().entities);
            system(&taken);
            self.world.borrow_mut().entities.extend(taken);
            // TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP TEMP
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
                    self.world.borrow_mut().get_updated_mesh_instances()
                ).expect("failed to create a Renderer")
            );
        }

        log!(info, "<<< dacho is initialized >>>");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = &mut self.renderer {
            //renderer.update_meshes(self.world.get_updated_mesh_instances())
                //.expect("failed to update meshes in the renderer");

            renderer.wait_for_device();
        }

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    #[expect(clippy::only_used_in_recursion,  reason = "WindowId")]
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
                if is_synthetic || event.repeat {
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

