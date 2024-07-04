// dacho/src/prelude/objects.rs

// crates
use serde::{Serialize, Deserialize};

// super
use super::types::V3;

#[derive(Clone, Serialize, Deserialize)]
pub enum Object {
    Camera (Camera),
    Shape  (Shape)
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Camera {
    Orthographic { position: V3, zoom: f32, aspect_ratio: f32, near: f32, far: f32 },
    Perspective  { position: V3, fov:  f32, aspect_ratio: f32, near: f32, far: f32 }
}

impl Camera {
    pub(crate) const DEFAULT_3D: Object = Object::Camera(Self::Perspective { position: V3::ONE, fov:  45.0, aspect_ratio: 16.0/9.0, near: 0.0001, far: 10000.0 });
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Shape {
    Cube   { position: V3, size:   V3  },
    Sphere { position: V3, radius: f32 }
}
