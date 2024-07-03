// dacho/src/prelude/shapes.rs

/* getters such as position or size, are only of use before running the World,
 * since an Object is just an instruction which gets "built" later into real geometry
 * or other types of objects like a camera
 */

// crates
use serde::{Serialize, Deserialize};

// super
use super::{
    colors::Color,
    materials::Material,
    types::{V2, V3}
};

// crate
use crate::application::camera::CameraMode;

// temp implementation
#[derive(Default)]
pub enum Anchor {
    Bottom = -1,
    #[default]
    Center,
    Top
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Object {
    Cube   (V3, V3,  V3, V2),
    Sphere (V3, f32, V3, V2),
    Camera (V3, CameraMode)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Camera {
    pub position: V3,
    pub mode:     CameraMode
}

impl Camera {
    #[must_use]
    pub const fn new(position: V3, mode: CameraMode) -> Self {
        Self { position, mode }
    }

    pub fn position(&mut self, rhs: V3) -> &mut Self {
        self.position = rhs;

        self
    }

    #[must_use]
    pub fn build(&self) -> Object {
        Object::Camera(self.position, self.mode.clone())
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(V3::Z * 10.0, CameraMode::default())
    }
}

pub struct Cube {
    pub position: V3,
    pub size:     V3,
    pub color:    V3,
    pub material: V2
}

#[allow(dead_code)]
impl Cube {
    #[must_use]
    pub fn new(position: V3, size: V3, anchor: Anchor, color: V3, material: V2) -> Self {
        Self {
            position: V3::new(
                position.x,
                (size.y * 0.5).mul_add(-(anchor as i32 as f32), position.y),
                position.z
            ),
            size,
            color,
            material
        }
    }

    pub fn position(&mut self, rhs: V3) -> &mut Self {
        self.position = rhs;

        self
    }

    pub fn size(&mut self, rhs: V3) -> &mut Self {
        self.size = rhs;

        self
    }

    pub fn color(&mut self, rhs: V3) -> &mut Self {
        self.color = rhs;

        self
    }

    pub fn material(&mut self, rhs: V2) -> &mut Self {
        self.material = rhs;

        self
    }

    pub fn anchor(&mut self, anchor: Anchor) -> &mut Self {
        self.position.y -= self.size.y * 0.5 * anchor as i32 as f32;

        self
    }

    #[must_use]
    pub const fn build(&self) -> Object {
        Object::Cube(self.position, self.size, self.color, self.material)
    }
}

#[allow(dead_code)]
impl Default for Cube {
    fn default() -> Self {
        Self::new(V3::ZERO, V3::ONE, Anchor::default(), Color::default(), Material::default())
    }
}

pub struct Sphere {
    pub position: V3,
    // refers to radius
    pub size:     f32,
    pub color:    V3,
    pub material: V2
}

#[allow(dead_code)]
impl Sphere {
    #[must_use]
    pub fn new(position: V3, size: f32, anchor: Anchor, color: V3, material: V2) -> Self {
        Self {
            position: V3::new(
                position.x,
                (size * 0.5).mul_add(-(anchor as i32 as f32), position.y),
                position.z
            ),
            size,
            color,
            material
        }
    }

    pub fn position(&mut self, rhs: V3) -> &mut Self {
        self.position = rhs;

        self
    }

    pub fn size(&mut self, rhs: f32) -> &mut Self {
        self.size = rhs;

        self
    }

    pub fn color(&mut self, rhs: V3) -> &mut Self {
        self.color = rhs;

        self
    }

    pub fn material(&mut self, rhs: V2) -> &mut Self {
        self.material = rhs;

        self
    }

    pub fn anchor(&mut self, anchor: Anchor) -> &mut Self {
        self.position.y -= self.size * anchor as i32 as f32;

        self
    }

    #[must_use]
    pub const fn build(&self) -> Object {
        Object::Sphere(self.position, self.size, self.color, self.material)
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(V3::ZERO, 0.5, Anchor::default(), Color::default(), Material::default())
    }
}

