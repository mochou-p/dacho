// dacho/src/main.rs

use anyhow::Result;

use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode::Escape, PhysicalKey::Code}
};

use dacho::application::Application;

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut application = Application::new(&event_loop)?;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event, is_synthetic: false, .. }, .. } => {
                if event.physical_key == Code(Escape) {
                    elwt.exit();
                }

                application.keyboard_input(&event);
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                application.mouse_input(&delta);
            },
            Event::AboutToWait => {
                application.renderer.wait_for_device();
                application.update();
                application.renderer.request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                application.redraw();
            },
            _ => ()
        }
    })?;

    Ok(())
}

