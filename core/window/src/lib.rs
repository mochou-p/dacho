// dacho/core/window/src/lib.rs

// crates
use {
    anyhow::Result,
    winit::{
        dpi::PhysicalSize,
        event_loop::ActiveEventLoop,
        window::Window as winit_Window
    }
};

// crate
use dacho_log::create_log;

pub struct Window {
        raw:    winit_Window,
    pub width:  u16,
    pub height: u16
}

impl Window {
    #[expect(clippy::missing_errors_doc, reason = "no docs")]
    pub fn new(
        title:      &str,
        width:       u16,
        height:      u16,
        event_loop: &ActiveEventLoop
    ) -> Result<Self> {
        create_log!(debug);

        let window_attributes = winit_Window::default_attributes()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height));

        let raw = event_loop.create_window(window_attributes)?;

        Ok(Self { raw, width, height })
    }

    #[must_use]
    pub const fn raw(&self) -> &winit_Window {
        &self.raw
    }

    #[inline]
    pub fn request_redraw(&self) {
        self.raw.request_redraw();
    }
}

