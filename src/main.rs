#![allow(dead_code)]
mod ui;
mod pulse;
pub use pulse::Pulse;
use std::sync::mpsc;

fn main() {
        ui::ui::ui_run();
    let (tx, rx) = mpsc::channel::<String>();
    let ctx = Pulse::initiate();
    ctx.set_subscribe();
    ctx.set_sink_callback(tx);
    ctx.run();
    // loop {
    //     println!("{} this", rx.recv().unwrap());
    // }
}
