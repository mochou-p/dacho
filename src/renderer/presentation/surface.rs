// dacho/src/renderer/presentation/surface.rs

// crates
use {
    anyhow::Result,
    ash::{extensions::khr, vk},
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle},
    winit::window::Window
};

// crate
use crate::renderer::{
    devices::Device,
    setup::{Entry, Instance},
    VulkanObject
};

// debug
#[cfg(debug_assertions)]
use crate::{
    app::logger::Logger,
    log
};

pub struct Surface {
    pub loader: khr::Surface,
        raw:    vk::SurfaceKHR
}

impl Surface {
    pub fn new(
        entry:    &Entry,
        instance: &Instance,
        window:   &Window
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating Surface");

        let loader = khr::Surface::new(entry.raw(), instance.raw());

        let raw = unsafe {
            ash_window::create_surface(
                entry.raw(),
                instance.raw(),
                window.raw_display_handle(),
                window.raw_window_handle(),
                None
            )
        }?;

        Ok(Self { loader, raw })
    }
}

impl VulkanObject for Surface {
    type RawType = vk::SurfaceKHR;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn destroy(&self, _: Option<&Device>) {
        #[cfg(debug_assertions)]
        log!(info, "Destroying Surface");

        unsafe { self.loader.destroy_surface(self.raw, None); }
    }
}

