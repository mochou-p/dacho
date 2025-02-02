// dacho/core/components/camera/src/lib.rs

use glam::{
    f32::{Mat4, Quat, Vec2, Vec3, Vec4},
    EulerRot
};


#[non_exhaustive]
pub struct Camera {
    pub view_updated: bool,
    pub proj_updated: bool,
    pub position:     Vec3,
    pub direction:    Quat,
    pub up:           Vec3,
    pub aspect_ratio: f32,
    pub near_far:     Vec2,
    pub screen:       Vec4,
    pub view:         Mat4,
    pub projection:   Mat4
}

impl Camera {
    #[inline]
    pub fn move_by(&mut self, delta: Vec3) {
        self.position     += delta;
        self.view_updated  = true;
    }

    #[inline]
    pub fn move_to(&mut self, position: Vec2) {
        self.position     = position.extend(self.position.z);
        self.view_updated = true;
    }

    #[inline]
    pub fn rotate_by(&mut self, delta: Vec3) {
        self.direction = Quat::from_euler(
            EulerRot::XYZ, delta.x, delta.y, delta.z
        ) * self.direction;

        self.view_updated = true;
    }

    #[inline]
    pub fn zoom_by(&mut self, delta: f32) {
        let ar_delta       = delta * self.aspect_ratio;
        self.screen       -= Vec4::new(-ar_delta, ar_delta, delta, -delta);
        self.proj_updated  = true;
    }

    #[inline]
    pub fn update_view(&mut self) {
        self.view = Mat4::look_at_rh(
            self.position,
            self.position + Vec3::from(
                self.direction.to_euler(EulerRot::XYZ)
            ),
            self.up
        );

        self.view_updated = false;
    }

    #[inline]
    pub fn update_projection(&mut self) {
        self.projection = Mat4::orthographic_rh(
            self.screen.x,   self.screen.y,
            self.screen.z,   self.screen.w,
            self.near_far.x, self.near_far.y
        );

        self.proj_updated = false;
    }

    #[inline]
    pub fn try_update(&mut self) {
        if self.view_updated {
            self.update_view();
        }

        if self.proj_updated {
            self.update_projection();
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let position     = Vec3::Z * 5.0;
        let direction    = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, -1.0);
        let up           = Vec3::Y;
        let near_far     = Vec2 { x: 0.1, y: 100.0 };
        let aspect_ratio = 16.0 / 9.0;

        let screen = Vec4::new(
            -0.5 * aspect_ratio,  0.5 * aspect_ratio,
             0.5,                -0.5
        );

        Self {
            view_updated: false,
            proj_updated: false,
            position,
            direction,
            up,
            aspect_ratio,
            near_far,
            screen,
            view: Mat4::look_at_rh(
                position,
                position + Vec3::from(
                    direction.to_euler(EulerRot::XYZ)
                ),
                up
            ),
            projection: Mat4::orthographic_rh(
                screen.x,   screen.y,
                screen.z,   screen.w,
                near_far.x, near_far.y
            )
        }
    }
}

