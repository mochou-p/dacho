// dacho/src/main.rs

use anyhow::Result;

use winit::{
    event::{DeviceEvent, Event, WindowEvent},
    event_loop::EventLoop
};

use dacho::renderer::Renderer;

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut renderer = Renderer::new(&event_loop)?;

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                elwt.exit();
            },
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event, is_synthetic: false, .. }, .. } => {
                renderer.keyboard_input(&event);
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                renderer.mouse_input(&delta);
            },
            Event::AboutToWait => {
                renderer.wait_for_device();
                renderer.update();
                renderer.request_redraw();
            },
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                renderer.redraw();
            },
            _ => ()
        }
    })?;

    Ok(())
}

