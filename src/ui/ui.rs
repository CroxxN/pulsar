use eframe::egui::{self, Ui};
// use libpulse_binding::volume::{VolumeLinear, VolumeDB};

struct PulseData {
    user_volume: f64,
    rx: std::sync::mpsc::Receiver<String>
    // volume: Volume 
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
        Box::new(|_cc| Box::new(PulseData{
            user_volume: 0.0,
            rx
        }),
    ));
    // let event_loop = eframe::EventLoopBuilder::new().build();
    // event_loop.(event_handler);
}


impl eframe::App for PulseData{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pulsar");
            if ui.add(egui::Slider::new(&mut self.user_volume, 0_f64..=1_f64)).changed() {
                make(ui);
            };
            ui.label(format!("The volume is: {}", self.user_volume));
            
            match self.rx.recv(){
                Ok(value) => {
                    ui.add(egui::Label::new(value));
                },
                Err(_) => {}
            }
        });   
    }
}

fn make(ui: &mut Ui){
    ui.add(egui::Label::new("This"));
}