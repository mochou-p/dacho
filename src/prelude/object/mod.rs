// dacho/src/prelude/object/mod.rs

// modules
pub mod camera;
#[allow(non_snake_case)]
pub mod shape2D;
#[allow(non_snake_case)]
pub mod shape3D;

// pub mod
pub use {camera::*, shape2D::*, shape3D::*};

// crates
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Object {
    Camera  (camera::InnerCamera),
    Shape2D (shape2D::InnerShape2D),
    Shape3D (shape3D::InnerShape3D)
}

