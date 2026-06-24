use eframe::egui;
use std::sync::mpsc;
use std::thread;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc as tmpsc;

struct ChatApp {
    tx: tmpsc::UnboundedSender<String>,
    rx: mpsc::Receiver<String>,

    input: String,
    chat: Vec<String>,
}

impl eframe::App for ChatApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        while let Ok(msg) = self.rx.try_recv() {
            self.chat.push(msg);
        }

        ui.heading("RustyChat");
        ui.separator();

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .max_height(ui.available_height() - 40.0)
            .show(ui, |ui| {
                for msg in &self.chat {
                    ui.label(msg);
                }
            });

        ui.separator();

        ui.horizontal(|ui| {
            let text = ui.text_edit_singleline(&mut self.input);

            let send = ui.button("Send").clicked()
                || (text.lost_focus()
                    && ui.input(|i| i.key_pressed(egui::Key::Enter)));

            if send && !self.input.is_empty() {
                let msg = format!("{}\n", self.input);

                let _ = self.tx.send(msg);

                self.input.clear();
                text.request_focus();
            }
        });

        ui.ctx().request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let (ui_tx, mut net_rx) = tmpsc::unbounded_channel::<String>();
    let (net_tx, ui_rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let stream = match TcpStream::connect("127.0.0.1:8080").await {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Connection failed: {e}");
                    return;
                }
            };

            let (reader, mut writer) = stream.into_split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    res = reader.read_line(&mut line) => {
                        if res.unwrap() == 0 {
                            break;
                        }

                        let _ = net_tx.send(line.clone());
                        line.clear();
                    }

                    Some(msg) = net_rx.recv() => {
                        let _ = writer.write_all(msg.as_bytes()).await;
                    }
                }
            }
        });
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "RustyChat",
        options,
        Box::new(move |_cc| {
            Ok(Box::new(ChatApp {
                tx: ui_tx,
                rx: ui_rx,
                input: String::new(),
                chat: Vec::new(),
            }))
        }),
    )
}
