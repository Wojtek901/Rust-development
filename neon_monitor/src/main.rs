use eframe::egui;
use sysinfo::System;

struct NeonMonitor{
    sys: System,
}

impl Default for NeonMonitor{
    fn default() -> Self{
        Self{
            sys: System::new_all(),
        }
    }
}

impl eframe::App for NeonMonitor{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui|{
            let cpu_name = self.sys.cpus()[0].brand();
            ui.heading(format!("System Monitor | {}", cpu_name));
        });

        egui::CentralPanel::default().show(ctx, |ui|{
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

            ui.separator();

            ui.heading("CPU usage");

            egui::ScrollArea::vertical().show(ui, |ui|{
                for (i, core) in self.sys.cpus().iter().enumerate(){
                    let usage = core.cpu_usage();

                    ui.horizontal(|ui|{
                        ui.add_space(10.0);

                        ui.allocate_ui(egui::vec2(60.0, 10.0), |ui|{
                            ui.label(format!("Core: {}", i));
                        });

                        let core_bar = egui::ProgressBar::new(usage / 100.0).text(format!("{:.1}%", usage));
                        ui.add(core_bar);
                    });
                }
            });
        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Neon Monitor",
        options,
        Box::new(|_cc| Box::<NeonMonitor>::default()),
    )
}
