use eframe::egui;
// use libpulse_binding::volume::{VolumeLinear, VolumeDB};

struct PulseData {
    user_volume: f64,
    rx: std::sync::mpsc::Receiver<String>,
    // Global audio data holder state
    labels: Vec<String>,
}

pub fn ui_run(rx: std::sync::mpsc::Receiver<String>) {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(250.0, 350.0)),
        // follow_system_theme: true,
        default_theme: eframe::Theme::Dark,
        // initial_window_pos: Some(epaint::Pos2{x: 500.0, y: 200.0}), // TODO: Change this to remember the user saved position
        ..Default::default()
    };
    eframe::run_native(
        "Pulsar",
        options,
        Box::new(|_cc| {
            Box::new(PulseData {
                user_volume: 0.0,
                rx,
                // TODO: Initiate with already existing audio devices
                labels: vec!["Hey".to_string(), "Hello".to_string()],
            })
        }),
    );
    // let event_loop = eframe::EventLoopBuilder::new().build();
    // event_loop.(event_handler);
}

impl eframe::App for PulseData {
    // TODO: Work on this function
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pulsar");
            ui.add(egui::Slider::new(&mut self.user_volume, 0_f64..=1_f64));
            ui.label(format!("The volume is: {}", self.user_volume));
            let value = self.rx.try_recv();
            match value {
                Ok(v) => {
                    // Push to the global state-audio-session(SAS) data holder
                    self.labels.push(v);
                    // egui::CentralPanel::default().show_inside(ui, |ui_aux| {
                    //     // ui_aux.label(value);
                    //     ui_aux.add(Label::new(value));
                    // });
                    // ctx.request_repaint();
                }
                Err(_) => {}
            }
            // For each discovered SAS data, render a label
            self.labels.iter().for_each(|l| {
                ui.label(l);
            });
        });
    }
}
