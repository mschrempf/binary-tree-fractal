mod tree;

use crate::tree::{EguiDrawable, Tree};
use egui::CentralPanel;
use egui::Slider;
use egui::Stroke;
use egui::Ui;
use egui::Window;
use emath::{pos2, remap, vec2, Pos2, Rect};
use std::time::Instant;
use std::{f32::consts::PI, time::Duration};

fn main() {
    let mut tree = tree::Tree::new(5);

    let opts = eframe::NativeOptions::default();

    let mut duration = Duration::ZERO;

    eframe::run_simple_native("Binary Tree", opts, move |ctx, _frame| {
        CentralPanel::default().show(ctx, |ui| {
            let start = std::time::Instant::now();
            tree.ui(ui);
            ui.label(format!("Frame took {} µs", duration.as_micros()));
            ui.label(format!(
                "There are {} branches to draw",
                tree.number_of_branches()
            ));
            duration = start.elapsed();
        });
    })
    .unwrap();
}
