//! Extension to `egui` for 3D drawings

pub mod camera;
pub mod utils;

pub use camera::Camera;
use egui::{Color32, Stroke};
// glam's types are part of our interface
// TODO: use mint? But then we'd have to convert every time ...
pub use glam;
pub use glam::Vec3;

use glam::{Mat4, Vec3Swizzles, Vec4Swizzles};

#[derive(Clone)]
pub struct Painter3D {
    transform: Transform,
    painter_2d: egui::Painter,
}

// TODO: enum allowing custom transforms
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    rect: egui::Rect,
    mat: Mat4,
    inverse: Mat4,
}

impl Transform {
    pub fn new(mat: Mat4, rect: egui::Rect) -> Self {
        Self {
            rect,
            inverse: mat.inverse(),
            mat,
        }
    }

    /// Returns egui coordinates and z value for the given point
    pub fn world_to_egui(&self, world: glam::Vec3) -> (egui::Vec2, f32) {
        // World to "device coordinates"
        let pre: glam::Vec4 = self.mat * world.extend(1.);

        // Perspective division
        let mut dc = pre.xyz() / pre.w;

        // Invert Y
        dc.y *= -1.0;

        // Map to screen coordinates
        let sc = (dc + 1.) / 2.;
        let sc = sc.xy() * glam::Vec2::new(self.rect.width(), self.rect.height());

        let sc: mint::Vector2<f32> = sc.into();
        let sc: egui::Vec2 = sc.into();

        (sc + self.rect.min.to_vec2(), dc.z)
    }

    pub fn egui_to_world(&self, _egui: egui::Vec2, _z: f32) -> glam::Vec3 {
        /*
        let egui: mint::Vector2<f32> = egui.into();
        let egui: glam::Vec2 = egui.into();
        let egui = egui.extend(z);
        (self.inverse * egui.extend(1.)).xyz()
        */
        todo!()
    }

    /*
    /// Returns a Transform which has the given transformation prepended
    pub fn prepend(&self, tf: Transform) -> Transform {
        Self::from(tf.mat * self.mat)
    }
    */
}

impl Painter3D {
    pub fn new(painter_2d: egui::Painter, transform: Transform) -> Self {
        Self {
            transform,
            painter_2d,
        }
    }

    pub fn line(&self, a: Vec3, b: Vec3, stroke: Stroke) {
        let Some(a) = self.transform(a) else { return };
        let Some(b) = self.transform(b) else { return };

        self.painter_2d.line_segment([a, b], stroke)
    }

    pub fn circle_filled(&self, center: Vec3, radius: f32, fill_color: impl Into<Color32>) {
        let Some(center) = self.transform(center) else {
            return;
        };
        self.painter_2d.circle_filled(center, radius, fill_color)
    }

    pub fn circle(&self, center: Vec3, radius: f32, stroke: impl Into<Stroke>) {
        let Some(center) = self.transform(center) else {
            return;
        };
        self.painter_2d.circle_stroke(center, radius, stroke)
    }

    pub fn text(
        &self,
        pos: Vec3,
        anchor: egui::Align2,
        text: impl ToString,
        font_id: egui::FontId,
        text_color: Color32,
    ) -> Option<egui::Rect> {
        self.transform(pos)
            .map(|pos| self.painter_2d.text(pos, anchor, text, font_id, text_color))
    }

    /// Transform a point in world coordinates to egui coordinates
    pub fn transform(&self, pt: Vec3) -> Option<egui::Pos2> {
        let (sc, z) = self.transform.world_to_egui(pt);

        (0.0..=1.0).contains(&z).then(|| sc.to_pos2())
    }

    pub fn internal_transform(&self) -> &Transform {
        &self.transform
    }

    /// Get egui's 2D painter
    pub fn egui(&self) -> &egui::Painter {
        &self.painter_2d
    }

    /*
    /// Returns a painter which has the given transformation prepended
    pub fn transform(&self, mat: Mat4) -> Self {
        Self {
            transform: self.transform.prepend(Transform::from(mat)),
            // Context is Arc underneath so this is cheap
            painter_2d: self.painter_2d.clone(),
        }
    }
    */
}

pub struct ThreeUi {
    painter: Painter3D,
}

impl ThreeUi {
    pub fn new(painter: egui::Painter, tf: Transform) -> Self {
        Self {
            painter: Painter3D::new(painter, tf),
        }
    }
    /*
    pub fn fly_to(&mut self, destination: Vec3) {
        todo!()
    }
    */
    pub fn painter(&self) -> &Painter3D {
        &self.painter
    }
}

/// Shortcut for the ThreeGui::show() with default options in an egui Frame.
pub fn threegui(ui: &mut egui::Ui, user_func: impl FnMut(&mut ThreeUi) + Sized) {
    egui::Frame::canvas(ui.style()).show(ui, |ui| {
        ThreeWidget::new(ui.next_auto_id()).show(ui, user_func)
    });
}

pub struct ThreeWidget {
    desired_size: egui::Vec2,
    id: egui::Id,
}

impl ThreeWidget {
    pub fn new(id: impl Into<egui::Id>) -> Self {
        Self {
            desired_size: egui::Vec2::new(500., 400.),
            id: id.into(),
        }
    }

    pub fn with_desired_size(mut self, size: egui::Vec2) -> Self {
        self.desired_size = size;
        self
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        mut user_func: impl FnMut(&mut ThreeUi) + Sized,
    ) -> egui::Response {
        // Allocate area to draw
        let resp = ui.allocate_response(self.desired_size, egui::Sense::click_and_drag());

        // Get some inputs for the camera
        let mut camera =
            ui.data_mut(|data| data.get_persisted_mut_or_default::<Camera>(self.id).clone());
        camera.handle_response(&resp, ui);

        // Modify the camera stored
        ui.data_mut(|data| {
            *data.get_persisted_mut_or_default(self.id) = camera;
            // NOTE: Cannot use handle_response here as it deadlocks due to Response's Context
        });

        let proj = camera.projection(resp.rect.width(), resp.rect.height());
        let camera_tf = proj * camera.view();
        let tf = Transform::new(camera_tf, resp.rect);

        let mut three_ui = ThreeUi::new(ui.painter_at(resp.rect), tf);

        user_func(&mut three_ui);

        resp
    }
}

// TODO: Emebed a egui ui inside threegui; ThreeUi::embed(&self ui: )
// VR mode too...
//
// TODO: Do an example with a single canvas, but multiple widgets laid out using egui!
// So it's 3D views laid out with egui!!
