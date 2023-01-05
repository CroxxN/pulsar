#![allow(dead_code)]
mod pulse;
mod ui;
pub use pulse::Pulse;
use std::{thread, sync::mpsc::channel};

fn main() {
    let (tx, rx) = channel::<String>();
    thread::spawn(move || {
        let ctx = Pulse::initiate();
        ctx.set_subscribe();
        ctx.set_sink_callback(tx);
        ctx.run();
    });

    ui::ui::ui_run(rx);
    // loop {
    //     println!("{} this", rx.recv().unwrap());
    // }
}
