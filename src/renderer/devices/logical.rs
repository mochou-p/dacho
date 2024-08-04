// dacho/src/renderer/devices/logical.rs

// crates
use {
    anyhow::Result,
    ash::{extensions::khr, vk}
};

// super
use super::PhysicalDevice;

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

pub struct Device {
        raw:   ash::Device,
    pub queue: vk::Queue
}

impl Device {
    pub fn new(
        instance:        &Instance,
        physical_device: &PhysicalDevice
    ) -> Result<Self> {
        #[cfg(debug_assertions)]
        log!(info, "Creating Device");

        let raw = {
            let extension_names = [khr::Swapchain::name().as_ptr()];

            let features = vk::PhysicalDeviceFeatures {
                tessellation_shader: 1,
                fill_mode_non_solid: 1,
                sample_rate_shading: 1,
                sampler_anisotropy:  1,
                ..Default::default()
            };

            let queue_create_infos = [
                vk::DeviceQueueCreateInfo::builder()
                    .queue_family_index(0)
                    .queue_priorities(&[1.0])
                    .build()
            ];

            let create_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&extension_names)
                .enabled_features(&features);

            unsafe { instance.raw().create_device(*physical_device.raw(), &create_info, None) }?
        };

        let queue = unsafe { raw.get_device_queue(0, 0) };

        Ok(Self { raw, queue })
    }

    pub fn wait(&self)  {
        unsafe { self.raw.device_wait_idle() }
            .expect("Device wait idle failed");
    }
}

impl VulkanObject for Device {
    type RawType = ash::Device;

    fn raw(&self) -> &Self::RawType {
        &self.raw
    }

    fn destroy(&self, _: Option<&Device>) {
        #[cfg(debug_assertions)]
        log!(info, "Destroying Device");

        unsafe { self.raw.destroy_device(None); }
    }
}

