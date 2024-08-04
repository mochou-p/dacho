// dacho/src/renderer/devices/physical.rs

// crates
use {
    anyhow::{Context, Result},
    ash::vk
};

// crate
use crate::renderer::{
    setup::Instance,
    VulkanObject
};

// debug
#[cfg(debug_assertions)]
use crate::{
    app::logger::Logger,
    log
};

pub struct PhysicalDevice {
    raw: vk::PhysicalDevice
}

impl PhysicalDevice {
    pub fn new(instance: &Instance) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Choosing PhysicalDevice");

        let raw = unsafe { instance.raw().enumerate_physical_devices() }?
            .into_iter()
            .next()
            .context("No physical devices")?;

        Ok(Self { raw })
    }
}

impl VulkanObject for PhysicalDevice {
    type RawType = vk::PhysicalDevice;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }
}

