// dacho/src/main.rs

use anyhow::Result;

use dacho::application::Application;

#[cfg(debug_assertions)]
use dacho::{
    application::logger::Logger,
    log, log_indent
};

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)] {
        println!();
        log!(info, "Creating EventLoop");
    }

    let     event_loop  = winit::event_loop::EventLoop::new()?;
    let mut application = Application::new(&event_loop)?;

    #[cfg(debug_assertions)] {
        println!();
        log!(info, "Running EventLoop");
        log_indent!(1);
    }

    event_loop.run(move |event, elwt| {
        application.handle_event(&event, elwt);
    })?;

    Ok(())
}

