//! Extension to `egui` for 3D drawings

mod camera;

pub use camera::Camera;
// glam's types are part of our interface
// TODO: use mint? But then we'd have to convert every time ...
pub use glam;
pub use glam::Vec3;

use glam::{Mat4, Vec4Swizzles, Mat3};

#[derive(Clone)]
pub struct Painter3D {
    transform: Transform,
    painter_2d: egui::Painter
}

// TODO: enum allowing custom transforms
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    mat: Mat4,
    inverse: Mat4,
}

impl Transform {
    pub fn identity() -> Self {
        Self::from(Mat4::IDENTITY)
    }

    pub fn perspective(fov_y_radians: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        Self::from(Mat4::perspective_rh(
            fov_y_radians,
            aspect_ratio,
            z_near,
            z_far,
        ))
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self::from(Mat4::orthographic_rh(left, right, bottom, top, near, far))
    }

    pub fn world_to_egui(&self, world: glam::Vec3) -> egui::Vec2 {
        let pre: glam::Vec4 = self.mat * world.extend(1.);

        dbg!(self.mat);

        // Perspective division
        let v = pre.xy() / pre.w;
        dbg!(v);

        let v: mint::Vector2<f32> = v.into();
        v.into()
    }

    pub fn egui_to_world(&self, egui: egui::Vec2, z: f32) -> glam::Vec3 {
        /*
        let egui: mint::Vector2<f32> = egui.into();
        let egui: glam::Vec2 = egui.into();
        let egui = egui.extend(z);
        (self.inverse * egui.extend(1.)).xyz()
        */
        todo!()
    }

    /// Returns a Transform which has the given transformation prepended
    pub fn prepend(&self, tf: Transform) -> Transform {
        Self::from(tf.mat * self.mat)
    }
}

impl From<Mat4> for Transform {
    fn from(mat: Mat4) -> Self {
        Self {
            inverse: mat.inverse(),
            mat,
        }
    }
}

impl Painter3D {
    pub fn new(painter_2d: egui::Painter, transform: Transform) -> Self {
        Self {
            transform: Transform::identity(),
            painter_2d,
        }
    }

    pub fn line(&self, a: Vec3, b: Vec3, stroke: egui::Stroke) {
        let a = self.transf(a);
        let b = self.transf(b);
        self.painter_2d.line_segment([a.to_pos2(), b.to_pos2()], stroke)
    }

    fn transf(&self, pt: Vec3) -> egui::Vec2 {
        self.transform.world_to_egui(pt)
    }

    /// Returns a painter which has the given transformation prepended
    pub fn transform(&self, mat: Mat4) -> Self {
        Self {
            transform: self.transform.prepend(Transform::from(mat)),
            // Context is Arc underneath so this is cheap
            painter_2d: self.painter_2d.clone(),
        }
    }
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
    max_size: egui::Vec2,
    id: egui::Id,
}

impl ThreeWidget {
    pub fn new(id: impl Into<egui::Id>) -> Self {
        Self {
            max_size: egui::Vec2::INFINITY,
            id: id.into(),
        }
    }

    pub fn with_max_size(mut self, size: egui::Vec2) -> Self {
        self.max_size = size;
        self
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        mut user_func: impl FnMut(&mut ThreeUi) + Sized,
    ) -> egui::Response {
        // Allocate area to draw
        let desired_size = ui.available_size().min(self.max_size);
        let resp = ui.allocate_response(desired_size, egui::Sense::click_and_drag());

        // Get some inputs for the camera
        let mut camera = ui.data_mut(|data| data.get_persisted_mut_or_default::<Camera>(self.id).clone());
        camera.handle_response(&resp, ui);

        // Modify the camera stored
        ui.data_mut(|data| {
            *data.get_persisted_mut_or_default(self.id) = camera;
            // NOTE: Cannot use handle_response here as it deadlocks due to Response's Context
        });

        // Get the transform that puts the model on the screen
        let rect_offset: mint::Vector2<f32> = resp.rect.min.to_vec2().into();
        let rect_offset: Mat4 = Mat4::from_mat3(Mat3::from_translation(rect_offset.into()));

        let proj = camera.projection(resp.rect.width(), resp.rect.height());
        let camera_tf = proj * camera.view();
        let tf = Transform::from(rect_offset * camera_tf);

        let mut three_ui = ThreeUi::new(ui.painter().clone(), tf);

        user_func(&mut three_ui);

        resp
    }
}

// TODO: Emebed a egui ui inside threegui; ThreeUi::embed(&self ui: )
// VR mode too...
//
// TODO: Do an example with a single canvas, but multiple widgets laid out using egui!
// So it's 3D views laid out with egui!!
