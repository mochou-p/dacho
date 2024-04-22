// dacho/src/main.rs

use anyhow::Result;

#[cfg(debug_assertions)]
use dacho::application::logger::Logger;

use dacho::application::Application;

fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    {
        println!();
        Logger::info("Creating EventLoop");
    }

    let     event_loop  = winit::event_loop::EventLoop::new()?;
    let mut application = Application::new(&event_loop)?;

    #[cfg(debug_assertions)]
    {
        println!();
        Logger::info("Running EventLoop");
        Logger::indent(1);
    }

    event_loop.run(move |event, elwt| {
        application.handle_event(&event, elwt);
    })?;

    Ok(())
}

