// dacho/src/main.rs

use anyhow::Result;

use winit::event_loop::EventLoop;

#[cfg(debug_assertions)]
use dacho::application::logger::Logger;

use dacho::application::Application;

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;

    let mut application = Application::new(&event_loop)?;

    #[cfg(debug_assertions)]
    {
        println!();
        Logger::info("Running EventLoop");
    }

    event_loop.run(move |event, elwt| {
        application.handle_event(&event, elwt);
    })?;

    Ok(())
}

