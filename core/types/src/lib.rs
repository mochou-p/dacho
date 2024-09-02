// dacho/core/types/src/lib.rs

// core
use core::ops::{Add, Mul, Neg, Sub};

// crates
use glam::f32::{Vec2, Vec3};

#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, PartialEq)]
pub struct V2 {
    pub x: f32,
    pub y: f32
}

impl V2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE:  Self = Self { x: 1.0, y: 1.0 };

    pub const X:    Self = Self { x: 1.0, y: 0.0 };
    pub const Y:    Self = Self { x: 0.0, y: 1.0 };

    #[inline]
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn extend(&self) -> V3 {
        V3 { x: self.x, y: self.y, z: 0.0 }
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        let n = Vec2 { x: self.x, y: self.y }.normalize();

        Self { x: n.x, y: n.y }
    }

    #[inline]
    #[must_use]
    pub fn normalise(&self) -> Self {
        self.normalize()
    }

    #[inline]
    #[must_use]
    pub fn reverse_y(&self) -> Self {
        Self { x: self.x, y: -self.y }
    }

    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    #[inline]
    #[must_use]
    pub const fn to_glam(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }

    #[inline]
    #[must_use]
    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl Mul<f32> for V2 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

#[allow(clippy::exhaustive_structs)]
#[derive(Copy, Clone, PartialEq)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl V3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE:  Self = Self { x: 1.0, y: 1.0, z: 1.0 };

    pub const X:    Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y:    Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z:    Self = Self { x: 0.0, y: 0.0, z: 1.0 };

    pub const XY:   Self = Self { x: 1.0, y: 1.0, z: 0.0 };
    pub const XZ:   Self = Self { x: 1.0, y: 0.0, z: 1.0 };
    pub const YZ:   Self = Self { x: 0.0, y: 1.0, z: 1.0 };

    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn normalize(&self) -> Self {
        let n = Vec3 { x: self.x, y: self.y, z: self.z }.normalize();

        Self { x: n.x, y: n.y, z: n.z }
    }

    #[inline]
    #[must_use]
    pub fn normalise(&self) -> Self {
        self.normalize()
    }

    #[inline]
    #[must_use]
    pub fn reverse_y(&self) -> Self {
        Self { x: self.x, y: -self.y, z: self.z }
    }

    #[inline]
    #[must_use]
    pub const fn from_tuple(tuple: (f32, f32, f32)) -> Self {
        Self { x: tuple.0, y: tuple.1, z: tuple.2 }
    }

    #[inline]
    #[must_use]
    pub const fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    #[inline]
    #[must_use]
    pub const fn to_glam(&self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }

    #[inline]
    #[must_use]
    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
}

impl Add for V3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Sub for V3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Mul<f32> for V3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: f32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<i16> for V3 {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i16) -> Self::Output {
        Self { x: self.x * f32::from(rhs), y: self.y * f32::from(rhs), z: self.z * f32::from(rhs) }
    }
}

impl Neg for V3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z }
    }
}

