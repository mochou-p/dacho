// dacho/core/app/src/lib.rs

extern crate alloc;

use core::pin::Pin;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, ControlFlow::Poll},
    window::WindowId
};

use {
    dacho_ecs::{Arguments, System, World},
    dacho_renderer::Renderer,
    dacho_window::Window
};

pub use winit::keyboard::KeyCode;


pub struct App {
        title:     String,
    pub world:     Pin<Box<World>>,
        window:    Option<Window>,
        renderer:  Option<Renderer>
}

impl App {
    #[must_use]
    pub fn new(title: &str) -> Self {
        let world = Box::pin(World::new());

        Self {
            title:    String::from(title),
            world,
            window:   None,
            renderer: None
        }
    }

    pub fn add<S, A>(&self, _system: S)
    where
        S: System<A>,
        A: Arguments
    {
        //
    }

    #[tokio::main]
    #[expect(clippy::missing_panics_doc, reason = "no docs")]
    pub async fn run(mut self) {
        let event_loop = EventLoop::new()
            .expect("failed to create an EventLoop");

        event_loop.set_control_flow(Poll);
        event_loop.run_app(&mut self).expect("failed to run the app in event loop");

        #[expect(clippy::unwrap_used, reason = "temp")]
        drop(self.renderer.unwrap());
    }
}

#[expect(clippy::unwrap_used, reason = "temp")]
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window.get_or_insert(
            Window::new(&self.title, 800, 600, event_loop)
                .expect("failed to create Window")
        );

        self.renderer.get_or_insert(
            Renderer::new(event_loop, self.window.as_ref().unwrap())
                .expect("failed to create Renderer")
        );
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.renderer.as_ref().unwrap().wait_for_device();
        self.window.as_ref().unwrap().request_redraw();
    }

    #[expect(clippy::renamed_function_params, reason = "winit reuses `event`")]
    fn window_event(
        &mut self,
        event_loop:   &ActiveEventLoop,
        _window_id:     WindowId,
        window_event:  WindowEvent
    ) {
        #[expect(clippy::wildcard_enum_match_arm, reason = "lots of unused winit events")]
        match window_event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.renderer.as_mut().unwrap().redraw(0.0);
            },
            _ => ()
        }
    }
}

