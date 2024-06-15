// dacho/src/application/camera.rs

// core
use core::f32::consts::{FRAC_PI_2, PI};

// crates
use {
    glam::f32 as glam,
    winit::{
        event::KeyEvent,
        keyboard::{KeyCode::*, PhysicalKey::Code}
    }
};

struct CameraRotation {
    angle: glam::Vec2
}

impl CameraRotation {
    fn to_direction(&self) -> glam::Vec3 {
        glam::Vec3::new(
            self.angle.y.sin() * self.angle.x.cos(),
            self.angle.x.sin(),
            self.angle.y.cos() * self.angle.x.cos()
        )
    }
}

struct CameraMovement {
    positive: glam::Vec3,
    negative: glam::Vec3
}

struct CameraSpeed {
    translation: f32,
    rotation:    f32
}

struct Bound {
    min: f32,
    max: f32
}

struct CameraBounds {
    rotation_x: Bound
}

pub struct Camera {
    translation: glam::Vec3,
    rotation:    CameraRotation,
    movement:    CameraMovement,
    speed:       CameraSpeed,
    bounds:      CameraBounds
}

impl Camera {
    pub const fn new(translation: glam::Vec3) -> Self {
        let rotation = CameraRotation {
            angle: glam::Vec2::new(0.0, PI)
        };

        let movement = CameraMovement {
            positive: glam::Vec3::ZERO,
            negative: glam::Vec3::ZERO
        };

        let speed = CameraSpeed {
            translation:  0.025,
            rotation:    -0.001
        };

        let bounds = CameraBounds {
            rotation_x: Bound {
                #[allow(clippy::approx_constant)]
                min: -1.570_796_3,
                max:  FRAC_PI_2
            }
        };

        Self {
            translation,
            rotation,
            movement,
            speed,
            bounds
        }
    }

    pub fn keyboard_input(&mut self, event: &KeyEvent) {
        if event.repeat {
            return;
        }

        let speed = self.speed.translation * (1 - event.state as i32) as f32;

        match event.physical_key {
            Code(KeyA)      => { self.movement.negative.x = speed; },
            Code(KeyD)      => { self.movement.positive.x = speed; },
            Code(KeyW)      => { self.movement.negative.z = speed; },
            Code(KeyS)      => { self.movement.positive.z = speed; },
            Code(ShiftLeft) => { self.movement.negative.y = speed; },
            Code(Space)     => { self.movement.positive.y = speed; },
            _ => ()
        }
    }

    pub fn mouse_motion(&mut self, delta: &(f64, f64)) {
        let x = delta.1 as f32 * self.speed.rotation;
        let y = delta.0 as f32 * self.speed.rotation;

        self.rotation.angle.y += y;

        self.rotation.angle.x = (self.rotation.angle.x - x)
            .clamp(
                f32::EPSILON.mul_add( 1000.0, self.bounds.rotation_x.min),
                f32::EPSILON.mul_add(-1000.0, self.bounds.rotation_x.max)
            );
    }

    pub fn transform(&mut self) -> (glam::Vec3, glam::Vec3) {
        self.translation -=
            glam::Quat::from_rotation_y(self.rotation.angle.y)
            * (self.movement.positive - self.movement.negative);

        (self.translation, self.rotation.to_direction())
    }
}

