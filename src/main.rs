// dacho/src/main.rs

use anyhow::Result;

use winit::{
    event::{Event, WindowEvent},
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
            Event::AboutToWait => {
                renderer.wait_for_device();
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

