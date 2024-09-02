// dacho/core/renderer/vulkan/backend/src/presentation/surface.rs

// crates
use {
    anyhow::Result,
    ash::{extensions::khr, vk},
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle},
    winit::window::Window
};

// crate
use crate::setup::{Entry, Instance};

use dacho_log::{create_log, destroy_log};

pub struct Surface {
    pub loader: khr::Surface,
    pub raw:    vk::SurfaceKHR
}

impl Surface {
    pub fn new(
        entry:    &Entry,
        instance: &Instance,
        window:   &Window
    ) -> Result<Self> {
        create_log!(debug);

        let loader = khr::Surface::new(&entry.raw, &instance.raw);

        let raw = unsafe {
            ash_window::create_surface(
                &entry.raw,
                &instance.raw,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None
            )
        }?;

        Ok(Self { loader, raw })
    }

    pub fn drop(&self) {
        destroy_log!(debug);

        unsafe { self.loader.destroy_surface(self.raw, None); }
    }
}

