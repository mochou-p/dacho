// dacho/src/prelude/mod.rs

// modules
pub(super) mod primitives;
pub(super) mod types;

// pub mod
pub use {
    super::ecs::*,
    types::*
};

// crates
use anyhow::Result;

// super
use super::application::Application;

// debug
#[cfg(debug_assertions)]
use super::{
    application::logger::Logger,
    log, log_indent
};

#[allow(clippy::missing_errors_doc)]
pub async fn dacho_main() -> Result<()> {
    #[cfg(debug_assertions)] {
        println!();
        log!(info, "Creating EventLoop");
    }

    let     event_loop  = winit::event_loop::EventLoop::new()?;
    let mut application = Application::new(&event_loop)?;

    #[cfg(debug_assertions)] {
        println!();
        log!(info, "Running EventLoop");
        log_indent!(true);
    }

    event_loop.run(move |event, elwt| {
        application.handle_event(&event, elwt);
    })?;

    Ok(())
}

