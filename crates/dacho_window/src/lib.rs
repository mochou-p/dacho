// dacho/crates/dacho_window/src/lib.rs

#![cfg_attr(debug_assertions, expect(clippy::print_stdout, reason = "FPS logging"))]

#[cfg(debug_assertions)]
use std::time::Instant;

use ash_window::enumerate_required_extensions;

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::raw_window_handle::HasDisplayHandle as _;
use winit::window::{Window, WindowId};

use dacho_renderer::{Renderer, Vulkan};

pub use winit;
pub use dacho_renderer as renderer;


struct App {
    first:    bool,
    #[cfg(debug_assertions)]
    timer:    Instant,
    #[cfg(debug_assertions)]
    fps:      u32,
    window:   Option<Window>,
    vulkan:   Option<Vulkan>,
    renderer: Option<Renderer>
}

impl App {
    #[cfg(debug_assertions)]
    #[inline]
    fn new() -> Self {
        Self {
            first:    true,
            timer:    Instant::now(),
            fps:      0,
            window:   None,
            vulkan:   None,
            renderer: None
        }
    }

    #[cfg(not(debug_assertions))]
    #[inline]
    const fn new() -> Self {
        Self {
            first:    true,
            window:   None,
            vulkan:   None,
            renderer: None
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.first {
            self.first = false;
        } else {
            return;
        }

        let window_attributes = Window::default_attributes()
            .with_title("dacho");
        let window = event_loop
            .create_window(window_attributes)
            .unwrap();

        let display_handle = event_loop
            .display_handle()
            .unwrap()
            .into();
        let required_extensions = enumerate_required_extensions(display_handle)
            .unwrap();
        let vulkan = Vulkan::new(required_extensions);

        let inner_size  = window.inner_size();
        let clear_color = [49.0/255.0, 50.0/255.0, 68.0/255.0, 1.0];
        let renderer    = vulkan.new_renderer(&window, inner_size.width, inner_size.height, clear_color);

        self.window   = Some(window);
        self.vulkan   = Some(vulkan);
        self.renderer = Some(renderer);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window
            .as_ref()
            .unwrap()
            .request_redraw();

        #[cfg(debug_assertions)]
        if self.timer.elapsed().as_secs() >= 1 {
            println!("{} FPS", self.fps);
            self.fps   = 0;
            self.timer = Instant::now();
        } else {
            self.fps += 1;
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let vulkan = self.vulkan
                    .as_mut()
                    .unwrap();

                let renderer = self.renderer
                    .as_mut()
                    .unwrap();

                let window = self.window
                    .as_ref()
                    .unwrap();

                vulkan.render(renderer, || window.pre_present_notify());
            },
            _ => ()
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        let vulkan = self.vulkan
            .take()
            .unwrap();

        let renderer = self.renderer
            .take()
            .unwrap();

        vulkan.device_wait_idle();
        vulkan.destroy_renderer(renderer);
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

