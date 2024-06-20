// dacho/src/application/window.rs

// crates
use {
    anyhow::Result,
    winit::{
        dpi::PhysicalSize,
        event_loop::EventLoop,
        window::{
            Window as winit_Window,
            WindowBuilder
        }
    }
};

// super
use super::logger::Logger;

// crate
use crate::log;

pub struct Window {
        raw:    winit_Window,
    pub width:  u32,
    pub height: u32
}

impl Window {
    pub fn new(
        title:      &str,
        width:       u32,
        height:      u32,
        event_loop: &EventLoop<()>
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating Window");

        let raw = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height))
            .build(event_loop)?;

        if raw.set_cursor_grab(winit::window::CursorGrabMode::Locked).is_err() {
            log!(warning, "Failed to lock the cursor");
        }

        raw.set_cursor_visible(false);

        Ok(Self { raw, width, height })
    }

    pub const fn raw(&self) -> &winit_Window {
        &self.raw
    }

    pub fn request_redraw(&self) {
        self.raw.request_redraw();
    }
}

