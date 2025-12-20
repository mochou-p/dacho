// dacho/crates/dacho_window/src/lib.rs

use std::ffi;

use ash_window::enumerate_required_extensions;

use winit::dpi::PhysicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::raw_window_handle::HasDisplayHandle as _;
use winit::window::Window as Handle;

pub use winit;


#[derive(Default)]
pub struct Window {
    pub initialised: bool,
    pub handle:      Option<Handle>,
    pub size:        PhysicalSize<u32>,
    pub clear_color: [f32; 4]
}

impl Window {
    pub fn initialise(&mut self, event_loop: &ActiveEventLoop) {
        if self.initialised {
            return;
        }
        self.initialised = true;

        let window_attributes = Handle::default_attributes()
            .with_inner_size(PhysicalSize::<u16> { width: 1500, height: 1000 })
            .with_resizable(false)
            .with_title("dacho");
        let window = event_loop
            .create_window(window_attributes)
            .unwrap();
        let inner_size = window.inner_size();

        self.handle = Some(window);
        self.size   = (inner_size.width, inner_size.height).into();
    }

    #[must_use]
    pub const fn handle(&self) -> &Handle {
        self.handle.as_ref().unwrap()
    }

    #[must_use]
    pub fn required_extensions(&self, event_loop: &ActiveEventLoop) -> &'static [*const ffi::c_char] {
        let display_handle = event_loop
            .display_handle()
            .unwrap()
            .into();

        enumerate_required_extensions(display_handle)
            .unwrap()
    }

    #[must_use]
    #[inline]
    pub fn resized(&mut self, new_size: PhysicalSize<u32>) -> bool {
        // NOTE: some compositors signal resizes even when the window size stays the same
        if self.size == new_size {
            return false;
        }

        self.size = new_size;
        true
    }

    #[inline]
    pub fn pre_present(&self) {
        self.handle().pre_present_notify();
    }

    #[inline]
    pub fn redraw(&self) {
        self.handle().request_redraw();
    }
}

