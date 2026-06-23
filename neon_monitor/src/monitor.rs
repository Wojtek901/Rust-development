use eframe::egui;
use sysinfo::System;
use egui_plot::{Line, Plot};
use std::time::{Instant, Duration};

const REFRESHING_TIME: u128 = 6;
const HISTORY_LENGTH: usize = 100;
const N_COLUMNS_FOR_CORE_USAGE: usize = 4;

struct FpsTracker {
    start: Instant,
    last_timestamp: Duration,
    cumulated_fps: f64,
    current_fps: f64,
}

struct CpuTracker {
    history: Vec<f32>,
    cores_usage: Vec<f32>,
    cumulated_global: f32,
    cumulated_cores: Vec<f32>,
}

pub struct NeonMonitor {
    sys: System,
    cpu: CpuTracker,
    fps: FpsTracker,
    frame_counter: u128,
}

impl Default for NeonMonitor {
    fn default() -> Self {
        let sys_data = System::new_all();
        let num_cores = sys_data.cpus().len();

        Self {
            sys: sys_data,
            cpu: CpuTracker {
                history: vec![0.0; HISTORY_LENGTH],
                cores_usage: vec![0.0; num_cores],
                cumulated_global: 0.0,
                cumulated_cores: vec![0.0; num_cores],
            },
            fps: FpsTracker {
                start: Instant::now(),
                last_timestamp: Duration::from_millis(0),
                cumulated_fps: 0.0,
                current_fps: 0.0,
            },
            frame_counter: 0,
        }
    }
}

impl NeonMonitor {
    fn draw_ram_section(&self, ui: &mut egui::Ui) {
        ui.heading("RAM memory");
        let total = self.sys.total_memory() as f32;
        let used = self.sys.used_memory() as f32;
        let fraction = if total > 0.0 { used / total } else { 0.0 };

        let gb_div = 1_073_741_824.0;
        let info = format!("{:.1} GB / {:.1} GB", used / gb_div, total / gb_div);
        ui.add(egui::ProgressBar::new(fraction).text(info));
    }

    fn draw_cpu_plot(&self, ui: &mut egui::Ui) {
        ui.heading("Full CPU usage");
        let points: Vec<[f64; 2]> = self.cpu.history.iter()
            .enumerate()
            .map(|(x, &y)| [x as f64, y as f64])
            .collect();

        let line = Line::new(points).fill(0.0);
        Plot::new("CPU plot").height(100.0).include_y(0.0).include_y(100.0).show(ui, |plot_ui| {
            plot_ui.line(line);
        });
    }

    fn draw_cores_usage(&self, ui: &mut egui::Ui) {
        ui.heading("CPU usage by core");
        ui.add_space(5.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("cores")
                .num_columns(N_COLUMNS_FOR_CORE_USAGE)
                .spacing([10.0, 10.0])
                .show(ui, |ui| {
                    for i in 0..self.sys.cpus().len() {
                        let usage = self.cpu.cores_usage[i];
                        let green_intensity = 50 + (usage * 2.0) as u8;
                        let bg_color = egui::Color32::from_rgb(0, green_intensity, 0);

                        egui::Frame::none()
                            .fill(bg_color)
                            .rounding(1.0)
                            .show(ui, |ui| {
                                ui.set_min_size(egui::vec2(60.0, 60.0));
                                ui.set_max_size(egui::vec2(60.0, 60.0));
                                ui.vertical_centered(|ui| {
                                    ui.add_space(10.0);
                                    ui.label(egui::RichText::new(format!("C{}", i)).color(egui::Color32::WHITE));
                                    ui.label(egui::RichText::new(format!("{:.0}%", usage)).strong().color(egui::Color32::WHITE));
                                });
                            });

                        if (i + 1) % N_COLUMNS_FOR_CORE_USAGE == 0 {
                            ui.end_row();
                        }
                    }
                })
        });
    }
}

impl eframe::App for NeonMonitor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.frame_counter += 1;
        let should_refresh = self.frame_counter % REFRESHING_TIME == 0;

        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        self.cpu.cumulated_global += self.sys.global_cpu_info().cpu_usage();
        for (i, core) in self.sys.cpus().iter().enumerate() {
            self.cpu.cumulated_cores[i] += core.cpu_usage();
        }

        let current_elapsed = self.fps.start.elapsed();
        let dt = current_elapsed - self.fps.last_timestamp;
        self.fps.last_timestamp = current_elapsed;
        self.fps.cumulated_fps += 1.0 / dt.as_secs_f64();

        if should_refresh {
            let divisor = REFRESHING_TIME as f32;

            self.cpu.history.push(self.cpu.cumulated_global / divisor);
            if self.cpu.history.len() > HISTORY_LENGTH {
                self.cpu.history.remove(0);
            }
            self.cpu.cumulated_global = 0.0;

            for i in 0..self.sys.cpus().len() {
                self.cpu.cores_usage[i] = self.cpu.cumulated_cores[i] / divisor;
                self.cpu.cumulated_cores[i] = 0.0;
            }

            self.fps.current_fps = self.fps.cumulated_fps / (REFRESHING_TIME as f64);
            self.fps.cumulated_fps = 0.0;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let cpu_name = self.sys.cpus()[0].brand();
                ui.heading(format!("System Monitor | {}", cpu_name));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.heading(format!("FPS: {:.1}", self.fps.current_fps));
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_ram_section(ui);
            ui.separator();
            self.draw_cpu_plot(ui);
            ui.separator();
            self.draw_cores_usage(ui);
        });

        ctx.request_repaint();
    }
}
