// dacho/core/renderer/vulkan/backend/src/devices/physical.rs

use {
    anyhow::{Context as _, Result},
    ash::vk
};

use crate::setup::Instance;

use dacho_log::create_log;


pub struct PhysicalDevice {
    pub raw: vk::PhysicalDevice
}

impl PhysicalDevice {
    pub fn new(instance: &Instance) -> Result<Self> {
        create_log!(debug);

        let raw = unsafe { instance.raw.enumerate_physical_devices() }?
            .into_iter()
            .next()
            .context("No physical devices")?;

        Ok(Self { raw })
    }
}

