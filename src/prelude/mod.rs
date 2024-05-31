// dacho/src/prelude/mod.rs

pub mod colors;
pub mod materials;
pub mod primitives;
pub mod shapes;
pub mod types;
pub mod world;

pub use {
    colors::{Color, Color as Colour},
    materials::Material,
    shapes::{Anchor, Cube, Sphere},
    types::{V2, V3},
    world::World
};

use anyhow::Result;

use super::application::{
    scene::Data,
    Application
};

#[cfg(debug_assertions)]
use super::{
    application::logger::Logger,
    log, log_indent
};

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
        log_indent!(1);
    }

    event_loop.run(move |event, elwt| {
        application.handle_event(&event, elwt);
    })?;

    Ok(())
}

