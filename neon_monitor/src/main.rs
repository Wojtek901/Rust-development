use eframe::egui;
use sysinfo::System;
use egui_plot::{Line, Plot};
use std::time::{Instant, Duration};

const REFRESHING_TIME: u8 = 6;
const HISTORY_LENGTH: usize = 100;

struct NeonMonitor{
    sys: System,
    cpu_history: Vec<f32>,
    cumulated_cpu_history: f32,
    cumulated_cores: Vec<f32>,
    cores_usage: Vec<f32>,
    start: Instant,
    last_timestamp: Duration,
    cumulated_fps: f64,
    fps: f64,
    fps_counter: u128,
}

impl Default for NeonMonitor{
    fn default() -> Self{
        let sys_data = System::new_all();
        let num_cores = sys_data.cpus().len();
        Self{
            sys: sys_data,
            cpu_history: vec![0.0;HISTORY_LENGTH],
            cumulated_cpu_history: 0.0,
            cumulated_cores: vec![0.0; num_cores],
            cores_usage: vec![0.0; num_cores],
            start: Instant::now(),
            last_timestamp: Duration::from_millis(0),
            cumulated_fps: 0.0,
            fps: 0.0,
            fps_counter: 0,
        }
    }
}

impl NeonMonitor{
    fn draw_ram_section(&self, ui: &mut egui::Ui){
        ui.heading("RAM memory");

        let total_ram_bytes = self.sys.total_memory() as f32;
        let used_ram_bytes = self.sys.used_memory() as f32;

        let ram_fraction = if total_ram_bytes > 0.0{used_ram_bytes / total_ram_bytes} else{0.0};

        let gb_fraction = 1_073_741_824.0;
        let total_gb = total_ram_bytes / gb_fraction;
        let used_gb = used_ram_bytes / gb_fraction;

        let ram_info = format!("{:.1} GB / {:.1} GB", used_gb, total_gb);
        let ram_bar = egui::ProgressBar::new(ram_fraction).text(ram_info);
        ui.add(ram_bar);
    }

    fn draw_cpu_plot(&self, ui: &mut egui::Ui){
        ui.heading("Full CPU usage");

        let points: Vec<[f64; 2]> = self.cpu_history.iter().enumerate().map(|(x, &y)| [x as f64, y as f64]).collect();
        let line = Line::new(points).fill(0.0);
        Plot::new("CPU plot").height(100.0).include_y(0.0).include_y(100.0).show(ui, |plot_ui|{
            plot_ui.line(line)
        });
    }

    fn draw_cores_usage(&mut self, ui: &mut egui::Ui){
        ui.heading("CPU usage by core");

        for (i, core) in self.sys.cpus().iter().enumerate(){
            self.cumulated_cores[i] += core.cpu_usage();
        }

        if self.fps_counter % REFRESHING_TIME as u128 == 0{
            for (i, _) in self.sys.cpus().iter().enumerate(){
                self.cores_usage[i] = self.cumulated_cores[i] / REFRESHING_TIME as f32;
                 self.cumulated_cores[i] = 0.0;
            }
        }

        egui::ScrollArea::vertical().show(ui, |ui|{
            for (i, _) in self.sys.cpus().iter().enumerate(){
                ui.horizontal(|ui|{
                    ui.add_space(10.0);

                    ui.allocate_ui(egui::vec2(60.0, 10.0), |ui|{
                        ui.label(format!("Core: {}", i));
                    });

                    let usage = self.cores_usage[i];
                    let core_bar = egui::ProgressBar::new(usage / 100.0).text(format!("{:.1}%", usage));
                    ui.add(core_bar);
                });
            }
        });
    }

    fn draw_fps(&mut self, ui: &mut egui::Ui){
        let current_elapse = self.start.elapsed();
        let dt = current_elapse - self.last_timestamp;
        self.last_timestamp = current_elapse;
        self.cumulated_fps += 1.0 / dt.as_secs_f64();

        if self.fps_counter % REFRESHING_TIME as u128 == 0{
            self.fps = self.cumulated_fps / REFRESHING_TIME as f64;
            self.cumulated_fps = 0.0;
        }

        ui.heading(format!("FPS: {:.1}", self.fps));
    }
}

impl eframe::App for NeonMonitor{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        let global_usage = self.sys.global_cpu_info().cpu_usage();
        self.cumulated_cpu_history += global_usage;

        if self.fps_counter % REFRESHING_TIME as u128 == 0{
            let cpu_usage_global = self.cumulated_cpu_history / REFRESHING_TIME as f32;
            self.cpu_history.push(cpu_usage_global);

            if self.cpu_history.len() > HISTORY_LENGTH{
                self.cpu_history.remove(0);
            }

            self.cumulated_cpu_history = 0.0;
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
            let cpu_name = self.sys.cpus()[0].brand();
            ui.heading(format!("System Monitor | {}", cpu_name));
        });

        egui::CentralPanel::default().show(ctx, |ui|{

            self.draw_ram_section(ui);
            ui.separator();

            self.draw_cpu_plot(ui);
            ui.separator();

            self.draw_cores_usage(ui);

            ui.separator();
            self.draw_fps(ui);
        });

        self.fps_counter += 1;
        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Neon Monitor",
        options,
        Box::new(|_cc| Box::<NeonMonitor>::default()),
    )
}
