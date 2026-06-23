#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod monitor;
use monitor::NeonMonitor;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native("Neon Monitor", options, Box::new(|_cc| Box::<NeonMonitor>::default()))
}
