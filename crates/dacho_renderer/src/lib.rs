// dacho/crates/dacho_renderer/src/lib.rs

#![expect(
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block,
    reason = "most of vulkan is unsafe"
)]

use core::ffi::c_char;

use ash::{Device, Entry, Instance};
use ash::vk::{ApplicationInfo, DeviceCreateInfo, DeviceQueueCreateInfo, InstanceCreateInfo, PhysicalDevice, Queue, API_VERSION_1_3};


pub struct Vulkan {
    _entry:           Entry,
    instance:         Instance,
    _physical_device: PhysicalDevice,
    device:           Device,
    _queue:           Queue
}

impl Vulkan {
    #[must_use]
    pub fn new(required_extensions: &[*const c_char]) -> Self {
        let entry = unsafe { Entry::load() }
            .unwrap();

        let application_info = ApplicationInfo::default()
            .api_version(API_VERSION_1_3);
        let instance_create_info = InstanceCreateInfo::default()
            .enabled_extension_names(required_extensions)
            .application_info(&application_info);
        let instance = unsafe { entry.create_instance(&instance_create_info, None) }
            .unwrap();

        let physical_device = unsafe { instance.enumerate_physical_devices() }
            .unwrap()
            .swap_remove(0);

        let device_queue_create_infos = [
            DeviceQueueCreateInfo::default()
                .queue_family_index(0)
                .queue_priorities(&[1.0])
        ];
        let device_create_info = DeviceCreateInfo::default()
            .queue_create_infos(&device_queue_create_infos);
        let device = unsafe { instance.create_device(physical_device, &device_create_info, None) }
            .unwrap();

        let queue = unsafe { device.get_device_queue(0, 0) };

        Self {
            _entry:           entry,
            instance,
            _physical_device: physical_device,
            device,
            _queue:           queue
        }
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

