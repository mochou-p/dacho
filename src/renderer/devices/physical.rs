// dacho/src/renderer/devices/physical.rs

use {
    anyhow::{Context, Result},
    ash::vk
};

use crate::renderer::{
    setup::instance::*,
    VulkanObject
};

#[cfg(debug_assertions)]
use crate::{
    application::logger::Logger,
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

