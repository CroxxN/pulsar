use eframe::{egui};
// use libpulse_binding::volume::{VolumeLinear, VolumeDB};

#[derive(Default)]
struct PulseData {
    user_volume: f64,
    // volume: Volume 
}

pub fn ui_run() {
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
        Box::new(|_cc| Box::new(PulseData::default())),
    );
    // let event_loop = eframe::EventLoopBuilder::new().build();
    // event_loop.(event_handler);
}


impl eframe::App for PulseData{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pulsar");
            ui.add(egui::Slider::new(&mut self.user_volume, 0_f64..=1_f64).text("Volume"));
            ui.label(format!("The volume is: {}", self.user_volume));

            // _frame.
            // ui.label(format!("{}", self.sink_list));
        });   
    }
}