// dacho/src/renderer/surface.rs

use anyhow::Result;

use ash::{extensions::khr, vk};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::window::Window;

use super::instance::Instance;

#[cfg(debug_assertions)]
use crate::application::logger::Logger;

pub struct Surface {
    pub loader: khr::Surface,
    pub raw:    vk::SurfaceKHR
}

impl Surface {
    pub fn new(
        entry:    &ash::Entry,
        instance: &Instance,
        window:   &Window
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        Logger::info("Creating Surface");

        let loader = khr::Surface::new(entry, &instance.raw);

        let raw = unsafe {
            ash_window::create_surface(
                entry,
                &instance.raw,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None
            )
        }?;

        Ok(Self { loader, raw })
    }

    pub fn destroy(&self) {
        #[cfg(debug_assertions)]
        Logger::info("Destroying Surface");

        unsafe { self.loader.destroy_surface(self.raw, None); }
    }
}

