// dacho/core/components/mesh/src/lib.rs

mod planar;
mod spatial;

use {
    ash::vk,
    glam::f32::{Mat4, Quat, Vec2, Vec3}
};


type MeshBuilder = dyn Fn() -> GeometryData;

#[derive(Clone)]
#[non_exhaustive]
pub struct GeometryData {
    pub shader:       String,
    pub id:           u32,
    pub cull_mode:    u32,
    pub polygon_mode: i32,
    pub vertices:     Vec<f32>,
    pub indices:      Vec<u32>,
    pub instances:    Vec<f32>
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
pub struct Mesh {
    pub children_ids:     Vec<u32>,
    pub parent_id_option: Option<u32>,
    #[expect(dead_code, reason = "currently there is only the default shader")]
        shader:           String,
    pub id:               u32, // for instancing
    pub nth:              u32, // for World operations
    pub model_matrix:     Mat4
}

impl Mesh {
    pub const BUILDERS: [&'static MeshBuilder; 4] = [
        &planar ::quad  ::mesh,
        &planar ::circle::mesh,
        &spatial::cube  ::mesh,
        &spatial::sphere::mesh
    ];

/*
    #[inline]
    #[must_use]
    pub fn pos(&self) -> V3 {
        let v3 = self.model_matrix.to_scale_rotation_translation().2;
        V3::new(v3.x, v3.y, v3.z)
    }

    #[inline]
    pub fn move_by<U: Fn(*mut Self)>(&mut self, rhs: V3, updater: U) {
        let (scale, rotation, mut translation) = self.model_matrix.to_scale_rotation_translation();

        translation += rhs.reverse_y().to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn move_to<U: Fn(*mut Self)>(&mut self, rhs: V3, updater: U) {
        let (scale, rotation, _) = self.model_matrix.to_scale_rotation_translation();

        let translation = rhs.reverse_y().to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn rotate_by<U: Fn(*mut Self)>(&mut self, axis_angle: V3, updater: U) {
        let (scale, mut rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        let angles = V3::from_tuple(rotation.to_euler(EulerRot::XYZ)) + axis_angle;

        rotation = Quat::from_euler(EulerRot::XYZ, angles.x, angles.y, angles.z);

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn rotate_to<U: Fn(*mut Self)>(&mut self, euler_xyz: V3, updater: U) {
        let (scale, _, translation) = self.model_matrix.to_scale_rotation_translation();

        let rotation = Quat::from_euler(EulerRot::XYZ, euler_xyz.x, euler_xyz.y, euler_xyz.z);

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn mirror<U: Fn(*mut Self)>(&mut self, axis: V3, updater: U) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        if axis.x == 1.0 { scale.x = -scale.x; }
        if axis.y == 1.0 { scale.y = -scale.y; }
        if axis.z == 1.0 { scale.z = -scale.z; }

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn scale_by<U: Fn(*mut Self)>(&mut self, rhs: V3, updater: U) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        scale += rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn scale_mul<U: Fn(*mut Self)>(&mut self, rhs: V3, updater: U) {
        let (mut scale, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        scale *= rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }

    #[inline]
    pub fn scale_to<U: Fn(*mut Self)>(&mut self, rhs: V3, updater: U) {
        let (_, rotation, translation) = self.model_matrix.to_scale_rotation_translation();

        let scale = rhs.to_glam();

        self.model_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

        updater(self);
    }
*/

    #[inline]
    #[must_use]
    pub fn quad(position: Vec3, size: Vec2) -> Self {
        let id = 0;

        let model_matrix = Mat4::from_scale_rotation_translation(
            size.extend(1.0),
            Quat::IDENTITY,
            position
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
    pub fn circle(position: Vec3, radius: f32) -> Self {
        let id = 1;

        let model_matrix = Mat4::from_scale_rotation_translation(
            Vec3 { x: radius, y: radius, z: 1.0 },
            Quat::IDENTITY,
            position
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
    pub fn cube(position: Vec3, size: f32) -> Self {
        let id = 2;

        let model_matrix = Mat4::from_scale_rotation_translation(
            Vec3::splat(size),
            Quat::IDENTITY,
            position
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
    pub fn sphere(position: Vec3, radius: f32) -> Self {
        let id = 3;

        let model_matrix = Mat4::from_scale_rotation_translation(
            Vec3::splat(radius),
            Quat::IDENTITY,
            position
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

