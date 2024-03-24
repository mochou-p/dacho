// dacho/src/main.rs

use anyhow::{Context, Result};

use ash::vk;

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder
};

fn main() -> Result<()> {
    let entry = unsafe { ash::Entry::load() }?;

    let instance = {
        let application_info = vk::ApplicationInfo::builder()
            .api_version(vk::API_VERSION_1_3);

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info);

        unsafe { entry.create_instance(&create_info, None) }?
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }?
        .into_iter()
        .next()
        .context("No physical devices")?;

    let device = {
        let queue_priorities = [1.0];

        let queue_create_infos = [
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(0)
                .queue_priorities(&queue_priorities)
                .build()
        ];

        let create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos);

        unsafe { instance.create_device(physical_device, &create_info, None) }?
    };

    let event_loop = EventLoop::new()?;

    let _window = WindowBuilder::new()
        .with_title("dacho")
        .build(&event_loop)?;

    event_loop.run(move |event, _| {
        match event {
            _ => ()
        }
    })?;

    unsafe {   device.destroy_device(None);   }
    unsafe { instance.destroy_instance(None); }

    Ok(())
}

