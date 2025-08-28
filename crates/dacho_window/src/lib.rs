// dacho/crates/dacho_window/src/lib.rs

use ash_window::enumerate_required_extensions;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::raw_window_handle::HasDisplayHandle as _;
use winit::window::{Window, WindowId};

use dacho_renderer::Vulkan;


struct App {
    window: Option<Window>,
    vulkan: Option<Vulkan>
}

impl App {
    const fn new() -> Self {
        Self { window: None, vulkan: None }
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

        let display_handle = event_loop
            .display_handle()
            .unwrap()
            .into();
        let required_extensions = enumerate_required_extensions(display_handle)
            .unwrap();
        let vulkan  = Vulkan::new(required_extensions);
        self.vulkan = Some(vulkan);
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

