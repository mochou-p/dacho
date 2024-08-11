// dacho/src/prelude/mesh/mod.rs

// modules
mod planar;

// crates
use glam::{EulerRot, f32::{Mat4, Quat, Vec3}};

// super
use super::types::{V2, V3};

// crate
use crate::{
    ecs::{component::Component, world::Id},
    renderer::rendering::GeometryData
};

type MeshBuilder = dyn Fn() -> GeometryData;

#[non_exhaustive]
pub struct Mesh {
    pub id:           Id, // for instancing
    pub model_matrix: Mat4
}

impl Component for Mesh {}

impl Mesh {
    pub(crate) const BUILDERS: [&'static MeshBuilder; 2] = [
        &planar::quad   ::mesh,
        &planar::circle ::mesh
    ];

    pub fn move_by(&mut self, rhs: V3) {
        let (scale, rotation, mut translation) = self.model_matrix.to_scale_rotation_translation();

        translation += rhs.reverse_y().to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    }

    pub fn move_to(&mut self, rhs: V3) {
        let (scale, rotation, _) = self.model_matrix.to_scale_rotation_translation();

        let translation = rhs.reverse_y().to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    }

    pub fn rotate_by(&mut self, axis_angle: V3) {
        let (scale, mut rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        let angles = V3::from_tuple(rotation.to_euler(EulerRot::XYZ)) + axis_angle;

        rotation = Quat::from_euler(EulerRot::XYZ, angles.x, angles.y, angles.z);

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    }

    pub fn rotate_to(&mut self, euler_xyz: V3) {
        let (scale, _, translation) = self.model_matrix.to_scale_rotation_translation();

        let rotation = Quat::from_euler(EulerRot::XYZ, euler_xyz.x, euler_xyz.y, euler_xyz.z);

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

    }

    pub fn mirror(&mut self, axis: V3) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        if (axis.x - 1.0).abs() < f32::EPSILON { scale.x *= -1.0; }
        if (axis.y - 1.0).abs() < f32::EPSILON { scale.y *= -1.0; }
        if (axis.z - 1.0).abs() < f32::EPSILON { scale.z *= -1.0; }

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    }

    pub fn scale_by(&mut self, rhs: V3) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        scale += rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    }

    pub fn scale_mul(&mut self, rhs: V3) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        scale *= rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    }

    pub fn scale_to(&mut self, rhs: V3) {
        let (_, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        let scale = rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);
    }

    #[must_use]
    pub fn quad(position: V3, size: V2) -> Self {
        let id = 0;

        let model_matrix = Mat4::from_scale_rotation_translation(
            (size * 0.5).to_glam().extend(1.0),
            Quat::IDENTITY,
            position.reverse_y().to_glam()
        );

        Self { id, model_matrix }
    }

    #[must_use]
    pub fn circle(position: V3, radius: f32) -> Self {
        let id = 1;

        let model_matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(radius, radius, 1.0),
            Quat::IDENTITY,
            position.reverse_y().to_glam()
        );

        Self { id, model_matrix }
    }
}

