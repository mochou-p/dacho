// dacho/src/prelude/types.rs

#[derive(Copy, Clone)]
pub struct V2 {
    pub x: f32,
    pub y: f32
}

impl V2 {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE:  Self = Self::new(1.0, 1.0);

    pub const X:    Self = Self::new(1.0, 0.0);
    pub const Y:    Self = Self::new(0.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn normalize(&self) -> Self {
        let n = glam::Vec2::from_array(self.to_array()).normalize();

        Self::new(n.x, n.y)
    }

    #[inline]
    pub fn normalise(&self) -> Self {
        self.normalize()
    }

    #[inline]
    pub fn to_array(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

#[derive(Copy, Clone)]
pub struct V3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl V3 {
    pub const ZERO: Self = Self::new( 0.0,  0.0,  0.0);
    pub const ONE:  Self = Self::new( 1.0,  1.0,  1.0);

    pub const X:    Self = Self::new(1.0, 0.0, 0.0);
    pub const Y:    Self = Self::new(0.0, 1.0, 0.0);
    pub const Z:    Self = Self::new(0.0, 0.0, 1.0);

    pub const XY:   Self = Self::new(1.0, 1.0, 0.0);
    pub const XZ:   Self = Self::new(1.0, 0.0, 1.0);
    pub const YZ:   Self = Self::new(0.0, 1.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(&self) -> Self {
        let n = glam::Vec3::from_array(self.to_array()).normalize();

        Self::new(n.x, n.y, n.z)
    }

    #[inline]
    pub fn normalise(&self) -> Self {
        self.normalize()
    }

    #[inline]
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

impl std::ops::Mul<isize> for V3 {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self { x: self.x * rhs as f32, y: self.y * rhs as f32, z: self.z * rhs as f32 }
    }
}

