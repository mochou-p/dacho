// dacho/crates/dacho_window/src/lib.rs

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};


struct App {
    window: Option<Window>
}

impl App {
    fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("dacho");
        let window = event_loop
            .create_window(window_attributes)
            .unwrap();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent) {
        event_loop.exit();
    }
}

pub fn main() {
    let mut app = App::new();

    let event_loop = EventLoop::new()
        .unwrap();

    event_loop
        .set_control_flow(ControlFlow::Poll);

    event_loop
        .run_app(&mut app)
        .unwrap();
}

