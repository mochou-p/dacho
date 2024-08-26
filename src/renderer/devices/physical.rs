// dacho/src/renderer/devices/physical.rs

// crates
use {
    anyhow::{Context, Result},
    ash::vk
};

// crate
use crate::{
    renderer::setup::Instance,
    create_log
};

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

