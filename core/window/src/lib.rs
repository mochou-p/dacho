// dacho/core/window/src/lib.rs

use {
    anyhow::Result,
    winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window as RawWindow}
};

use dacho_log::create_log;


#[non_exhaustive]
pub struct Window {
    pub raw:    RawWindow,
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

        let window_attributes = RawWindow::default_attributes()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height));

        let raw = event_loop.create_window(window_attributes)?;

        Ok(Self { raw, width, height })
    }

    #[inline]
    pub fn request_redraw(&self) {
        self.raw.request_redraw();
    }
}

