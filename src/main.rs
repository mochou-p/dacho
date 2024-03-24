// dacho/src/main.rs

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;

    let _window = WindowBuilder::new()
        .with_title("dacho")
        .build(&event_loop)?;

    event_loop.run(move |event, _| {
        match event {
            _ => ()
        }
    })?;

    Ok(())
}

