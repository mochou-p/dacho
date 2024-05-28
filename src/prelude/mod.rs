// dacho/src/prelude/mod.rs

pub mod primitives;
    mod shapes;
    mod types;
pub mod world;

pub use {
    shapes::Object::*,
    types::{V2, V3},
    world::World
};

use anyhow::Result;

use super::application::Application;

#[cfg(debug_assertions)]
use super::{
    application::logger::Logger,
    log, log_indent
};

#[inline]
pub fn run(world: &World) {
    dacho_main(world)
        .expect("failed to run dacho_main");
}

#[tokio::main]
async fn dacho_main(world: &World) -> Result<()> {
    #[cfg(debug_assertions)] {
        println!();
        log!(info, "Creating EventLoop");
    }

    let     event_loop  = winit::event_loop::EventLoop::new()?;
    let mut application = Application::new(&event_loop, world)?;

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

