// dacho/core/components/camera/src/lib.rs

use core::f32::consts::FRAC_PI_2;

use glam::{
    f32::{Mat4, Quat, Vec2, Vec3, Vec4},
    EulerRot
};


#[non_exhaustive]
pub enum Camera {
    TwoD {
        view_updated: bool,
        proj_updated: bool,
        position:     Vec3,
        rotation:     Vec3,
        up:           Vec3,
        aspect_ratio: f32,
        near_far:     Vec2,
        screen:       Vec4,
        view:         Mat4,
        projection:   Mat4
    },
    ThreeD {
        view_updated: bool,
        proj_updated: bool,
        position:     Vec3,
        rotation:     Vec3,
        aspect_ratio: f32,
        near_far:     Vec2,
        fov_y:        f32,
        view:         Mat4,
        projection:   Mat4
    }
}

impl Camera {
    #[inline]
    #[must_use]
    pub fn get_position(&self) -> &Vec3 {
        match self {
            Self::TwoD     { position, .. }
            | Self::ThreeD { position, .. } => {
                position
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn get_rotation(&self) -> &Vec3 {
        match self {
            Self::TwoD     { rotation, .. }
            | Self::ThreeD { rotation, .. } => {
                rotation
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn get_view(&self) -> &Mat4 {
        match self {
            Self::TwoD     { view, .. }
            | Self::ThreeD { view, .. } => {
                view
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn get_projection(&self) -> &Mat4 {
        match self {
            Self::TwoD     { projection, .. }
            | Self::ThreeD { projection, .. } => {
                projection
            }
        }
    }

    // world coords mode
    #[inline]
    pub fn move_by(&mut self, delta: Vec3) {
        match self {
            Self::TwoD     { position, view_updated, .. }
            | Self::ThreeD { position, view_updated, .. } => {
                *position     += delta;
                *view_updated  = true;
            }
        }
    }

    // minecraft mode
    #[inline]
    pub fn y_angle_relative_move_by(&mut self, delta: Vec3) {
        match self {
            Self::TwoD     { position, rotation, view_updated, .. }
            | Self::ThreeD { position, rotation, view_updated, .. } => {
                let quat       = Quat::from_euler(EulerRot::XYZ, 0.0, rotation.y, 0.0);
                *position     += quat * delta;
                *view_updated  = true;
            }
        }
    }

    // plane mode
    #[inline]
    pub fn rotation_relative_move_by(&mut self, delta: Vec3) {
        match self {
            Self::TwoD     { position, rotation, view_updated, .. }
            | Self::ThreeD { position, rotation, view_updated, .. } => {
                let quat       = Quat::from_euler(EulerRot::YXZ, rotation.y, rotation.x, rotation.z);
                *position     += quat * delta;
                *view_updated  = true;
            }
        }
    }

    #[inline]
    pub fn move_to(&mut self, rhs: Vec3) {
        match self {
            Self::TwoD     { position, view_updated, .. }
            | Self::ThreeD { position, view_updated, .. } => {
                *position     = rhs;
                *view_updated = true;
            }
        }
    }

    // expects 0/1 as axis masks
    #[inline]
    pub fn move_to_masked(&mut self, rhs: Vec3, mask: Vec3) {
        match self {
            Self::TwoD     { position, view_updated, .. }
            | Self::ThreeD { position, view_updated, .. } => {
                *position     = *position * flip_binary_vec3(mask) + rhs * mask;
                *view_updated = true;
            }
        }
    }

    #[inline]
    pub fn rotate_by(&mut self, delta: Vec3) {
        match self {
            Self::TwoD     { rotation, view_updated, .. }
            | Self::ThreeD { rotation, view_updated, .. } => {
                rotation.x  = (rotation.x + delta.x).clamp(-FRAC_PI_2, FRAC_PI_2);
                rotation.y += delta.y;
                rotation.z += delta.z;

                *view_updated = true;
            },
        }
    }

    #[inline]
    pub fn try_update_view(&mut self) {
        match self {
            Self::TwoD     { position, rotation, view, view_updated, .. }
            | Self::ThreeD { position, rotation, view, view_updated, .. } => {
                if !*view_updated {
                    return;
                }

                let quat = Quat::from_euler(EulerRot::YXZ, rotation.y, rotation.x, rotation.z);

                let forward = quat * Vec3::Z;
                let up      = quat * Vec3::Y;

                *view = Mat4::look_at_rh(
                    *position,
                    *position + forward,
                    up
                );
            }
        }
    }

    #[inline]
    pub fn try_update_projection(&mut self) {
        match self {
            Self::TwoD { near_far, screen, projection, proj_updated, .. } => {
                if !*proj_updated {
                    return;
                }

                *projection = Mat4::orthographic_rh(
                    screen.x,   screen.y,
                    screen.z,   screen.w,
                    near_far.x, near_far.y
                );

                *proj_updated = false;
            },
            Self::ThreeD { aspect_ratio, near_far, fov_y, projection, proj_updated, .. } => {
                if !*proj_updated {
                    return;
                }

                *projection = Mat4::perspective_rh(
                    *fov_y,
                    *aspect_ratio,
                    near_far.x,
                    near_far.y
                );

                *proj_updated = false;
            }
        }
    }

    #[inline]
    pub fn try_update(&mut self) {
        self.try_update_view();
        self.try_update_projection();
    }

    #[must_use]
    pub fn default_2d() -> Self {
        let position     = Vec3::Z * -5.0;
        let rotation     = Vec3::ZERO;
        let quat         = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
        let forward      = quat * Vec3::Z;
        let up           = quat * Vec3::Y;
        let near_far     = Vec2 { x: 0.1, y: 100.0 };
        let aspect_ratio = 16.0 / 9.0;
        let screen       = Vec4::new(
            0.5 * aspect_ratio, -0.5 * aspect_ratio, // what, i should probably not use rh
            0.5,                -0.5
        );

        Self::TwoD {
            view_updated: false,
            proj_updated: false,
            position,
            rotation,
            up,
            aspect_ratio,
            near_far,
            screen,
            view: Mat4::look_at_rh(
                position,
                position + forward,
                up
            ),
            projection: Mat4::orthographic_rh(
                screen.x,   screen.y,
                screen.z,   screen.w,
                near_far.x, near_far.y
            )
        }
    }

    #[must_use]
    pub fn default_3d() -> Self {
        let position     = Vec3::Z * -5.0;
        let rotation     = Vec3::ZERO;
        let quat         = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, 0.0);
        let forward      = quat * Vec3::Z;
        let up           = quat * Vec3::Y;
        let near_far     = Vec2 { x: 0.1, y: 100.0 };
        let aspect_ratio = 16.0 / 9.0;
        let fov_y        = 90.0_f32.to_radians();

        Camera::ThreeD {
            view_updated: false,
            proj_updated: false,
            position,
            rotation,
            aspect_ratio,
            near_far,
            fov_y,
            view: Mat4::look_at_rh(
                position,
                position + forward,
                up
            ),
            projection: Mat4::perspective_rh(
                -fov_y,
                aspect_ratio,
                near_far.x,
                near_far.y
            )
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::default_3d()
    }
}

#[inline]
#[must_use]
fn flip_binary_vec3(value: Vec3) -> Vec3 {
    (value - Vec3::ONE).abs()
}

