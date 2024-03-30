// dacho/src/renderer/surface.rs

use anyhow::Result;

use ash::{extensions::khr, vk};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::window::Window;

pub struct Surface {
    pub loader:  khr::Surface,
    pub surface: vk::SurfaceKHR
}

impl Surface {
    pub fn new(
        entry:    &ash::Entry,
        instance: &ash::Instance,
        window:   &Window
    ) -> Result<Self> {
        let loader = khr::Surface::new(&entry, &instance);

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.raw_display_handle(),
                window.raw_window_handle(),
                None
            )
        }?;

        Ok(
            Self {
                loader,
                surface
            }
        )
    }

    pub fn destroy(&self) {
        unsafe { self.loader.destroy_surface(self.surface, None); }
    }
}

