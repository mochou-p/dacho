// dacho/src/prelude/mod.rs

    mod shapes;
    mod types;
pub mod user_scene;

pub use {
    shapes::Object::*,
    types::V3,
    user_scene::UserScene as Scene
};

use anyhow::Result;

use {
    user_scene::UserScene,
    super::application::Application
};

#[cfg(debug_assertions)]
use super::{
    application::logger::Logger,
    log, log_indent
};

#[inline]
pub fn run(scene: &UserScene) {
    dacho_main(scene)
        .expect("failed to run dacho_main");
}

#[tokio::main]
async fn dacho_main(scene: &UserScene) -> Result<()> {
    #[cfg(debug_assertions)] {
        println!();
        log!(info, "Creating EventLoop");
    }

    let     event_loop  = winit::event_loop::EventLoop::new()?;
    let mut application = Application::new(&event_loop, scene)?;

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

