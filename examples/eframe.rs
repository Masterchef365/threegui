// Copied from https://docs.rs/eframe/0.24.1/eframe/index.html

use eframe::egui;
use egui::{Color32, Stroke};
use glam::Vec3;

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            threegui::threegui(ui, |three| {
                let paint = three.painter();

                let k = 10;
                let f = k as f32;
                for i in -k..=k {
                    paint.line(
                        Vec3::new(-1., 0., i as f32 / f),
                        Vec3::new(1., 0., i as f32 / f),
                        Stroke::new(0.5, Color32::WHITE),
                    );

                    paint.line(
                        Vec3::new(i as f32 / f, 0., -1.),
                        Vec3::new(i as f32 / f, 0., 1.),
                        Stroke::new(0.5, Color32::WHITE),
                    );
                }

                paint.line(
                    Vec3::new(1., 0.5, 0.9),
                    Vec3::new(-1., -0.1, 0.3),
                    Stroke::new(3., Color32::RED),
                );
                paint.line(
                    Vec3::new(0.3, -0.1, -0.3),
                    Vec3::new(0.2, -0.4, 0.9),
                    Stroke::new(1., Color32::LIGHT_BLUE),
                );

                paint.circle_filled(Vec3::new(0.3, -0.1, -0.3), 4.0, Color32::LIGHT_BLUE);

                paint.circle(
                    Vec3::new(0.2, -0.4, 0.9),
                    4.0,
                    Stroke::new(1., Color32::LIGHT_BLUE),
                );
            })
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .unwrap();
}

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}
