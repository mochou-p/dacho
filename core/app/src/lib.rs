// dacho/core/app/src/lib.rs

extern crate alloc;

mod timer;

use alloc::rc::Rc;
use core::{cell::RefCell, mem::take};
use std::collections::HashMap;

use winit::{
    application::ApplicationHandler,
    event::{MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow::Poll},
    keyboard::PhysicalKey,
    window::WindowId
};

use timer::Timer;

use dacho_components::*;
use dacho_ecs::{entity::Entity, world::World, query::{Query, QueryFn, QueryTuple}};
use dacho_renderer::Renderer;
use dacho_log::{log, create_log};
use dacho_window::Window;

pub use winit::keyboard::KeyCode;


#[non_exhaustive]
pub enum Schedule {
    Start,
    Update,
    Keyboard,
    MouseMovement,
    MouseButton,
    MouseWheel
}

type System = Box<dyn FnMut(&[Rc<Entity>])>;

struct Systems {
    start:          Vec<System>,
    update:         Vec<System>,
    keyboard:       Vec<System>,
    mouse_movement: Vec<System>,
    mouse_button:   Vec<System>,
    mouse_wheel:    Vec<System>
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

        let systems = Systems {
            start:          vec![],
            update:         vec![],
            keyboard:       vec![],
            mouse_movement: vec![],
            mouse_button:   vec![],
            mouse_wheel:    vec![]
        };

        let timer = Timer::new();

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
        use Schedule::*;

        match schedule {
            Start         => &mut self.systems.start,
            Update        => &mut self.systems.update,
            Keyboard      => &mut self.systems.keyboard,
            MouseMovement => &mut self.systems.mouse_movement,
            MouseButton   => &mut self.systems.mouse_button,
            MouseWheel    => &mut self.systems.mouse_wheel
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
        use Schedule::*;

        for system in match schedule {
            Start         => &mut self.systems.start,
            Update        => &mut self.systems.update,
            Keyboard      => &mut self.systems.keyboard,
            MouseMovement => &mut self.systems.mouse_movement,
            MouseButton   => &mut self.systems.mouse_button,
            MouseWheel    => &mut self.systems.mouse_wheel
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

            let mut get_meshes = |(query,): (Query<(MeshComponent,)>,)| {
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
        self.world.borrow_mut().spawn((self.timer.get_component(),));
        self.run_system_schedule(Schedule::Update);

        let mut w = self.world.borrow_mut();
        if let Some(i) = w.entities.iter().position(|entity| entity.get_component::<TimeComponent>().is_some()) {
            w.entities.remove(i);
        }

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
                        return self.window_event(
                            event_loop,
                            window_id,
                            WindowEvent::CloseRequested
                        );
                    }

                    self.world.borrow_mut().spawn((KeyComponent { code, down: event.state.is_pressed() },));
                    self.run_system_schedule(Schedule::Keyboard);

                    let mut w = self.world.borrow_mut();
                    if let Some(i) = w.entities.iter().position(|entity| entity.get_component::<KeyComponent>().is_some()) {
                        w.entities.remove(i);
                    }
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.world.borrow_mut().spawn((MousePositionComponent { x: position.x, y: position.y },));
                self.run_system_schedule(Schedule::MouseMovement);

                let mut w = self.world.borrow_mut();
                if let Some(i) = w.entities.iter().position(|entity| entity.get_component::<MousePositionComponent>().is_some()) {
                    w.entities.remove(i);
                }
            },
            WindowEvent::MouseInput { button, state, .. } => {
                if let MouseButton::Other(_) = button {
                    return;
                }

                self.world.borrow_mut().spawn((MouseButtonComponent { button, down: state.is_pressed() },));
                self.run_system_schedule(Schedule::MouseButton);

                let mut w = self.world.borrow_mut();
                if let Some(i) = w.entities.iter().position(|entity| entity.get_component::<MouseButtonComponent>().is_some()) {
                    w.entities.remove(i);
                }
            },
            WindowEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(x, y), .. } => {
                self.world.borrow_mut().spawn((MouseWheelComponent { x, y },));
                self.run_system_schedule(Schedule::MouseWheel);

                let mut w = self.world.borrow_mut();
                if let Some(i) = w.entities.iter().position(|entity| entity.get_component::<MouseWheelComponent>().is_some()) {
                    w.entities.remove(i);
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

