// dacho/src/renderer/device.rs

use anyhow::{Context, Result};

use ash::{extensions::khr, vk};

use super::instance::Instance;

pub struct PhysicalDevice {
    pub raw: vk::PhysicalDevice
}

impl PhysicalDevice {
    pub fn new(instance: &Instance) -> Result<Self> {
        let raw = unsafe { instance.raw.enumerate_physical_devices() }?
            .into_iter()
            .next()
            .context("No physical devices")?;

        Ok(Self { raw })
    }
}

pub struct Device {
    pub raw:   ash::Device,
    pub queue: vk::Queue
}

impl Device {
    pub fn new(
        instance:        &Instance,
        physical_device: &PhysicalDevice
    ) -> Result<Self> {
        let raw = {
            let queue_priorities = [1.0];

            let extension_names = [
                khr::Swapchain::name()
                    .as_ptr()
            ];

            let queue_create_infos = [
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(0)
                    .queue_priorities(&queue_priorities)
                    .build()
            ];

            let create_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&extension_names);

            unsafe { instance.raw.create_device(physical_device.raw, &create_info, None) }?
        };

        let queue = unsafe { raw.get_device_queue(0, 0) };

        Ok(Self { raw, queue })
    }

    pub fn wait(&self)  {
        unsafe { self.raw.device_wait_idle() }
            .expect("Device wait idle failed");
    }

    pub fn destroy(&self) {
        unsafe { self.raw.destroy_device(None); }
    }
}

