// dacho/src/main.rs

use anyhow::{Context, Result};

use ash::{
    extensions::khr::Surface,
    vk
};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let entry = unsafe { ash::Entry::load() }?;

    let instance = {
        let application_info = vk::ApplicationInfo::builder()
            .api_version(vk::API_VERSION_1_3);

        let extension_names = ash_window::enumerate_required_extensions(
            event_loop.raw_display_handle()
        )?;

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(extension_names);

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

    let _queue = unsafe { device.get_device_queue(0, 0) };

    let window = WindowBuilder::new()
        .with_title("dacho")
        .build(&event_loop)?;

    let surface_khr = unsafe { ash_window::create_surface(
        &entry,
        &instance,
        window.raw_display_handle(),
        window.raw_window_handle(),
        None
    ) }?;

    let surface = Surface::new(&entry, &instance);

    event_loop.run(move |event, _| {
        match event {
            _ => ()
        }
    })?;

    unsafe {  surface.destroy_surface(surface_khr, None); };
    unsafe {   device.destroy_device(None);               };
    unsafe { instance.destroy_instance(None);             };

    Ok(())
}

