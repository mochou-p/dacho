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

    let _physical_device = unsafe { instance.enumerate_physical_devices() }?
        .into_iter()
        .next()
        .context("No physical devices")?;

    let event_loop = EventLoop::new()?;

    let _window = WindowBuilder::new()
        .with_title("dacho")
        .build(&event_loop)?;

    event_loop.run(move |event, _| {
        match event {
            _ => ()
        }
    })?;

    unsafe { instance.destroy_instance(None); }

    Ok(())
}

