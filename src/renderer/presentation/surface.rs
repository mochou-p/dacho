// dacho/src/renderer/presentation/surface.rs

// crates
use {
    anyhow::Result,
    ash::{extensions::khr, vk},
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle},
    winit::window::Window
};

// crate
use crate::{
    renderer::{
        setup::{Entry, Instance},
        VulkanObject,
        LOG_SRC
    },
    debug
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
        debug!(LOG_SRC, "Creating Surface");

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

    fn destroy(&self) {
        debug!(LOG_SRC, "Destroying Surface");

        unsafe { self.loader.destroy_surface(self.raw, None); }
    }
}

