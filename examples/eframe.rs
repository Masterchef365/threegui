// Copied from https://docs.rs/eframe/0.24.1/eframe/index.html

use std::time::Instant;

use eframe::egui;
use egui::{Color32, Stroke};
use glam::Vec3;
use threegui::utils;

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            threegui::threegui(ui, |three| {
                let paint = three.painter();

                // Grid underneath it all
                utils::grid(paint, 10, 1.0, egui::Stroke::new(1.0, Color32::DARK_GRAY));

                // Text
                let pt = Vec3::new(-0.5, 0.5, -0.5);
                paint.text(
                    pt,
                    egui::Align2::LEFT_CENTER,
                    " Heyo",
                    egui::FontId::default(),
                    Color32::GOLD,
                );
                // Circle
                paint.circle_filled(pt, 2.0, Color32::GOLD);

                // Rectangle
                let pt = Vec3::new(0.5, -0.5, -0.5);
                if let Some(pos) = paint.transform(pt) {
                    let rect = egui::Rect::from_min_size(pos, egui::Vec2::new(30., 30.));
                    paint
                        .egui()
                        .rect(rect, egui::Rounding::ZERO, Color32::BLUE, Stroke::NONE);
                }

                // Red line
                paint.line(
                    Vec3::new(1., 0.5, 0.9),
                    Vec3::new(-1., -0.1, 0.3),
                    Stroke::new(3., Color32::RED),
                );

                //let pt = Vec3::new(-0.5, 0.5, 0.5);

                // Blue line
                let a = Vec3::new(self.start.elapsed().as_secs_f32().cos(), 0.1, -0.3);
                let b = Vec3::new(0.2, 0.4, 0.9);

                paint.line(a, b, Stroke::new(1., Color32::LIGHT_BLUE));

                paint.line(
                    Vec3::new(-1., -0.5, 0.9),
                    Vec3::new(-1., 0.1, -0.3),
                    Stroke::new(5., Color32::GREEN),
                );

                // Blue line's end caps
                paint.circle_filled(a, 4.0, Color32::LIGHT_BLUE);

                paint.circle(b, 4.0, Stroke::new(1., Color32::LIGHT_BLUE));
            })
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
    .unwrap();
}

struct MyEguiApp {
    start: Instant,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}
