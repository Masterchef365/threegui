// Copied from https://docs.rs/eframe/0.24.1/eframe/index.html

use eframe::egui;
use egui::{Color32, Stroke};
use glam::Vec3;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            threegui::threegui(ui, |three| {
                let paint = three.painter();
                paint.line(
                    Vec3::new(1., 0.5, 0.9),
                    Vec3::new(-1., -0.1, 0.3),
                    Stroke::new(1., Color32::RED),
                );
                paint.line(
                    Vec3::new(0.3, -0.1, -0.3),
                    Vec3::new(0.2, -0.4, 0.9),
                    Stroke::new(1., Color32::LIGHT_BLUE),
                );
            })
        });
    }
}
