// dacho/src/renderer/device.rs

use anyhow::Result;

use ash::{extensions::khr, vk};

pub struct Device {
    pub device: ash::Device,
    pub queue:  vk::Queue
}

impl Device {
    pub fn new(
        instance:        &ash::Instance,
        physical_device: &vk::PhysicalDevice
    ) -> Result<Self> {
        let device = {
            let queue_priorities = [
                1.0
            ];

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

            unsafe { instance.create_device(*physical_device, &create_info, None) }?
        };

        let queue = unsafe { device.get_device_queue(0, 0) };

        Ok(
            Self {
                device,
                queue
            }
        )
    }

    pub fn wait(&self)  {
        unsafe { self.device.device_wait_idle() }
            .expect("Device wait idle failed");
    }

    pub fn destroy(&self) {
        unsafe { self.device.destroy_device(None); }
    }
}

