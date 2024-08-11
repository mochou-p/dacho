// dacho/src/game/window.rs

// crates
use {
    anyhow::Result,
    winit::{
        dpi::PhysicalSize,
        event_loop::ActiveEventLoop,
        window::Window as winit_Window
    }
};

#[cfg(debug_assertions)]
use {
    super::logger::Logger,
    crate::log
};

pub struct Window {
        raw:    winit_Window,
    pub width:  u16,
    pub height: u16
}

impl Window {
    #[allow(clippy::missing_errors_doc)]
    pub fn new(
        title:      &str,
        width:       u16,
        height:      u16,
        event_loop: &ActiveEventLoop
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating Window");

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

