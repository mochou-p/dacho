// dacho/core/components/mesh/src/lib.rs

mod planar;
// mod spatial;

use {
    ash::vk,
    glam::{EulerRot, f32::{Mat4, Quat, Vec3}}
};

use dacho_types::{V2, V3};


type MeshBuilder = dyn Fn() -> GeometryData;

#[derive(Clone)]
#[non_exhaustive]
pub struct GeometryData {
    pub shader:       String,
    pub id:           u32,
    pub cull_mode:    u32,
    pub polygon_mode: i32,
    pub vertices:     Vec<f32>,
    pub instances:    Vec<f32>,
    pub indices:      Vec<u32>
}

impl GeometryData {
    #[inline]
    pub const fn new(
        shader:       String,
        id:           u32,
        cull_mode:    vk::CullModeFlags,
        polygon_mode: vk::PolygonMode,
        vertices:     Vec<f32>,
        instances:    Vec<f32>,
        indices:      Vec<u32>
    ) -> Self {
        Self {
            shader,
            id,
            cull_mode:       cull_mode.as_raw(),
            polygon_mode: polygon_mode.as_raw(),
            vertices,
            instances,
            indices
        }
    }
}

#[non_exhaustive]
pub struct MeshComponent {
    pub children_ids:     Vec<u32>,
    pub parent_id_option: Option<u32>,
    #[expect(dead_code, reason = "currently there is only the default shader")]
        shader:           String,
    pub id:               u32, // for instancing
    pub nth:              u32, // for World operations
    pub model_matrix:     Mat4
}

impl MeshComponent {
    pub const BUILDERS: [&'static MeshBuilder; 2] = [
        &planar::quad   ::mesh,
        &planar::circle ::mesh
    ];

    #[inline]
    pub fn move_by<U: Fn(*const Self)>(&mut self, rhs: V3, updater: U) {
        let (scale, rotation, mut translation) = self.model_matrix.to_scale_rotation_translation();

        translation += rhs.reverse_y().to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn move_to<U: Fn(*const Self)>(&mut self, rhs: V3, updater: U) {
        let (scale, rotation, _) = self.model_matrix.to_scale_rotation_translation();

        let translation = rhs.reverse_y().to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn rotate_by<U: Fn(*const Self)>(&mut self, axis_angle: V3, updater: U) {
        let (scale, mut rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        let angles = V3::from_tuple(rotation.to_euler(EulerRot::XYZ)) + axis_angle;

        rotation = Quat::from_euler(EulerRot::XYZ, angles.x, angles.y, angles.z);

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn rotate_to<U: Fn(*const Self)>(&mut self, euler_xyz: V3, updater: U) {
        let (scale, _, translation) = self.model_matrix.to_scale_rotation_translation();

        let rotation = Quat::from_euler(EulerRot::XYZ, euler_xyz.x, euler_xyz.y, euler_xyz.z);

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn mirror<U: Fn(*const Self)>(&mut self, axis: V3, updater: U) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        if axis.x == 1.0 { scale.x = -scale.x; }
        if axis.y == 1.0 { scale.y = -scale.y; }
        if axis.z == 1.0 { scale.z = -scale.z; }

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn scale_by<U: Fn(*const Self)>(&mut self, rhs: V3, updater: U) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        scale += rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn scale_mul<U: Fn(*const Self)>(&mut self, rhs: V3, updater: U) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        scale *= rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn scale_to<U: Fn(*const Self)>(&mut self, rhs: V3, updater: U) {
        let (_, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        let scale = rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    #[must_use]
    pub fn quad(position: V3, size: V2) -> Self {
        let id = 0;

        let model_matrix = Mat4::from_scale_rotation_translation(
            (size * 0.5).to_glam().extend(1.0),
            Quat::IDENTITY,
            position.reverse_y().to_glam()
        );

        Self {
            children_ids:     vec![],
            parent_id_option: None,
            shader:           String::from("default"),
            id,
            nth:              0,
            model_matrix
        }
    }

    #[inline]
    #[must_use]
    pub fn circle(position: V3, radius: f32) -> Self {
        let id = 1;

        let model_matrix = Mat4::from_scale_rotation_translation(
            Vec3::new(radius, radius, 1.0),
            Quat::IDENTITY,
            position.reverse_y().to_glam()
        );

        Self {
            children_ids:     vec![],
            parent_id_option: None,
            shader:           String::from("default"),
            id,
            nth:              0,
            model_matrix
        }
    }
}

