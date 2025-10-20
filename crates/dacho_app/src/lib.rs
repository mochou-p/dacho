// dacho/crates/dacho_app/src/lib.rs

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

use dacho_renderer::{Meshes, Renderer, Vulkan};
use dacho_window::{winit, Window};

pub use dacho_renderer as renderer;
pub use dacho_window   as window;


pub trait GameTrait: Default {
    fn  setup(&mut self) -> Option<Meshes> { None }
    fn update(&mut self)                   {      }
}

#[derive(Default)]
pub struct App<G: GameTrait> {
        game:     G,
        window:   Window,
        vulkan:   Option<Vulkan>,
        renderer: Option<Renderer>,
    pub meshes:   Option<Meshes>
}

impl<G: GameTrait> App<G> {
    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self).unwrap();
    }
}

impl<G: GameTrait> ApplicationHandler for App<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.initialised {
            return;
        }

        self.meshes = self.game.setup();

        self.window.initialise(event_loop);

        let required_extensions = self.window.required_extensions(event_loop);

        let vulkan   = Vulkan::new(required_extensions);
        let renderer = vulkan.new_renderer(
            self.window.handle(),
            self.window.size.width,
            self.window.size.height,
            self.window.clear_color,
            self.meshes.take().unwrap_or_default()
        );

        self.vulkan   = Some(vulkan);
        self.renderer = Some(renderer);
    }

    #[inline]
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.game.update();
    }

    #[inline]
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let vulkan   = self.vulkan  .as_ref().unwrap();
                let renderer = self.renderer.as_mut().unwrap();

                vulkan.render(renderer, || self.window.pre_present());
            },
            WindowEvent::Resized(new_size) => {
                if !self.window.resized(new_size) {
                    return;
                }

                let vulkan   = self.vulkan  .as_ref().unwrap();
                let renderer = self.renderer.as_mut().unwrap();

                vulkan.resize(renderer, new_size.width, new_size.height);
            },
            _ => ()
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        let vulkan   = self.vulkan  .take().unwrap();
        let renderer = self.renderer.take().unwrap();

        vulkan.device_wait_idle();
        vulkan.destroy_renderer(renderer);
    }
}

