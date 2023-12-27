//! Extension to `epaint` for 3D drawings

use glam::{Mat4, Vec3, Vec4Swizzles};
use egui::epaint;

pub struct Painter3D {
    transform: Transform,
}

// TODO: enum allowing custom transforms
pub struct Transform(Mat4, Mat4);

impl Transform {
    pub fn perspective(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        Self::from_matrix(Mat4::perspective_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self::from_matrix(Mat4::orthographic_rh(left, right, bottom, top, near, far))
    }

    fn from_matrix(mat: Mat4) -> Self {
        Self(mat, mat.inverse())
    }

    pub fn world_to_epaint(&self, world: glam::Vec3) -> epaint::Vec2 {
        let v: mint::Vector2<f32> = (self.0 * world.extend(1.)).xy().into();
        v.into()
    }

    pub fn epaint_to_world(&self, epaint: epaint::Vec2, z: f32) -> glam::Vec3 {
        let epaint: mint::Vector2<f32> = epaint.into();
        let epaint: glam::Vec2 = epaint.into();
        let epaint = epaint.extend(z);
        (self.1 * epaint.extend(1.)).xyz()
    }
}

impl Painter3D {

}
