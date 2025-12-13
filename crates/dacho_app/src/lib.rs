// dacho/crates/dacho_app/src/lib.rs

use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::event::{KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;

use dacho_renderer::{Meshes, Renderer, Vulkan};
use dacho_window::{winit, Window};

pub use dacho_renderer as renderer;
pub use dacho_window   as window;


pub trait GameTrait: Default {
    // execution flow ---------------------------------------------------
    fn    setup(&mut self) -> Meshes;
    fn   update(&mut self, _meshes: &mut Renderer, _delta_time: f32) {}
    fn  exiting(&mut self)                                           {}

    // window events ----------------------------------------------------
    fn  resized(&mut self, _width: u32, _height: u32) {}
    fn    moved(&mut self, _x:     i32, _y:      i32) {}
    fn  focused(&mut self, _value: bool)              {}
    fn  hovered(&mut self, _value: bool)              {}
    fn occluded(&mut self, _value: bool)              {}

    // input handling ---------------------------------------------------
    // TODO: gilrs
    fn keyboard(&mut self, _event:  KeyEvent,    _is_synthetic: bool) {}
    fn    mouse(&mut self, _button: MouseButton, _is_pressed:   bool) {}
    fn   cursor(&mut self, _x:      f64,         _y:            f64)  {}
    fn   scroll(&mut self, _x:      f32,         _y:            f32)  {}
}

#[derive(Default)]
pub struct App<G: GameTrait> {
    timer:    Option<Instant>,
    window:   Window,
    vulkan:   Option<Vulkan>,
    renderer: Option<Renderer>,
    game:     G
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

        self.timer = Some(Instant::now());

        self.window.initialise(event_loop);

        let required_extensions = self.window.required_extensions(event_loop);

        let vulkan   = Vulkan::new(required_extensions);
        let renderer = vulkan.new_renderer(
            self.window.handle(),
            self.window.size.width,
            self.window.size.height,
            self.window.clear_color,
            self.game.setup()
        );

        self.vulkan   = Some(vulkan);
        self.renderer = Some(renderer);
    }

    #[inline]
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let timer    = self.timer   .as_mut().unwrap();
        let renderer = self.renderer.as_mut().unwrap();

        let delta_time = timer.elapsed().as_secs_f32();
        *timer         = Instant::now();

        self.game  .update(renderer, delta_time);
        self.window.redraw();
    }

    #[inline]
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            // window -------------------------------------------------------------------
            WindowEvent::CloseRequested => {
                self.game.exiting();
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let vulkan   = self.vulkan  .as_ref().unwrap();
                let renderer = self.renderer.as_mut().unwrap();

                vulkan.render(renderer, || { self.window.pre_present() });
            },
            WindowEvent::Resized(new_size) => {
                if !self.window.resized(new_size) {
                    return;
                }

                let vulkan   = self.vulkan  .as_ref().unwrap();
                let renderer = self.renderer.as_mut().unwrap();

                vulkan.resize(renderer, new_size.width, new_size.height);
                self.game.resized(new_size.width, new_size.height);
            },
            WindowEvent::Moved(position) => {
                // NOTE: does this need a check for the same value like Resized?
                //       (cant test on wayland)
                self.game.moved(position.x, position.y);
            },
            WindowEvent::Focused(value) => {
                self.game.focused(value);
            },
            WindowEvent::CursorEntered { .. } => {
                self.game.hovered(true);
            },
            WindowEvent::CursorLeft { .. } => {
                self.game.hovered(false);
            },
            WindowEvent::Occluded(value) => {
                self.game.occluded(value);
            },
            // input --------------------------------------------------------------------
            WindowEvent::KeyboardInput { event: key_event, is_synthetic, .. } => {
                self.game.keyboard(key_event, is_synthetic);
            },
            WindowEvent::MouseInput { state, button, .. } => {
                self.game.mouse(button, state.is_pressed());
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.game.cursor(position.x, position.y);
            },
            WindowEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(x, y), .. } => {
                self.game.scroll(x, y);
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

