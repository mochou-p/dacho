// dacho/core/app/src/lib.rs

// modules
mod timer;

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
use dacho_ecs::world::World;
use dacho_renderer::Renderer;
use dacho_log::{log, create_log};
use dacho_window::Window;

// pub use
pub use winit::{
    dpi::PhysicalPosition,
    event::{MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::KeyCode as Key
};

pub struct App {
        title:    String,
    pub world:    World,
        timer:    Timer,
        window:   Option<Window>,
        renderer: Option<Renderer>
}

impl App {
    #[must_use]
    pub fn new(title: &str) -> Self {
        create_log!(info);

        let world = World::new();

        let timer = Timer::new(
            #[cfg(debug_assertions)]
            50
        );

        Self {
            title:    String::from(title),
            world,
            timer,
            window:   None,
            renderer: None
        }
    }

    #[tokio::main]
    #[allow(clippy::missing_panics_doc)]
    pub async fn run(mut self) {
        log!(info, "Running App");

        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        event_loop.set_control_flow(Poll);
        event_loop.run_app(&mut self).expect("failed to run the app in event loop");

        log!(info, "<<< dacho is shutting down >>>");

        if let Some(renderer) = self.renderer {
            drop(renderer);
        }
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
                    self.world.get_updated_mesh_instances()
                ).expect("failed to create a Renderer")
            );

            log!(info, "<<< dacho is initialized >>>");
        } 
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

    #[allow(clippy::only_used_in_recursion, clippy::renamed_function_params)]
    fn window_event(
        &mut self,
        event_loop:   &ActiveEventLoop,
        window_id:     WindowId,
        window_event:  WindowEvent
    ) {
        #[allow(clippy::wildcard_enum_match_arm)]
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

