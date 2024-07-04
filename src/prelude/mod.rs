// dacho/src/prelude/mod.rs

// modules
pub(super) mod colors;
pub(super) mod materials;
pub(super) mod objects;
pub(super) mod primitives;
pub(super) mod types;
pub(super) mod world;

// pub mod
pub use {
    colors::{Color as Colour, *},
    materials::*,
    objects::{Object::*, Camera::*, Shape::*},
    types::*,
    world::*
};

// crates
use anyhow::Result;

// super
use super::application::{
    scene::Data,
    Application
};

// debug
#[cfg(debug_assertions)]
use super::{
    application::logger::Logger,
    log, log_indent
};

#[allow(clippy::missing_errors_doc)]
pub async fn dacho_main(data: &Data) -> Result<()> {
    #[cfg(debug_assertions)] {
        println!();
        log!(info, "Creating EventLoop");
    }

    let     event_loop  = winit::event_loop::EventLoop::new()?;
    let mut application = Application::new(&event_loop, data)?;

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

