use egui::{Color32, Stroke};
pub use glam::Vec3;

use crate::Painter3D;

pub fn grid(paint: &Painter3D, n: usize, scale: f32, stroke: Stroke) {
    let k = 10;
    let f = k as f32;
    for i in -k..=k {
        paint.line(
            Vec3::new(-1., 0., i as f32 / f) * scale,
            Vec3::new(1., 0., i as f32 / f) * scale,
            stroke,
        );

        paint.line(
            Vec3::new(i as f32 / f, 0., -1.) * scale,
            Vec3::new(i as f32 / f, 0., 1.) * scale,
            stroke,
        );
    }
}
