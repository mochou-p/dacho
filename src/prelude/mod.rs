// dacho/src/prelude/mod.rs

pub use Object::*;

use anyhow::Result;

use super::application::Application;

#[cfg(debug_assertions)]
use super::{
    application::logger::Logger,
    log, log_indent
};

#[inline]
pub fn run(scene: &Scene) {
    start(scene).expect("failed to run start");
}

#[tokio::main]
async fn start(scene: &Scene) -> Result<()> {
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

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl V3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

impl From<[f32; 3]> for V3 {
    fn from(value: [f32; 3]) -> Self {
        Self { x: value[0], y: value[1], z: value[2] }
    }
}

impl std::ops::Sub for V3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl std::ops::Mul<f32> for V3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

#[derive(Debug)]
pub enum Object {
    Cube(V3, V3),
    Sphere(V3, f32)
}

pub struct Scene {
    pub objects: Vec<Object>
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, object: Object) -> &mut Self {
        self.objects.push(object);

        self
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}

