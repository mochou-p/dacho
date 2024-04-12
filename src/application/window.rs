// dacho/src/application/window.rs

use anyhow::Result;

use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{
        Window as winit_Window,
        WindowBuilder
    }
};

pub struct Window {
    pub window: winit_Window,
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
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height))
            .build(event_loop)?;

        window.set_cursor_grab(winit::window::CursorGrabMode::Locked)?;
        window.set_cursor_visible(false);

        Ok(
            Self {
                window,
                width,
                height
            }
        )
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }
}

