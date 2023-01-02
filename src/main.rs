#![allow(dead_code)]
mod pulse;
mod ui;
pub use pulse::Pulse;
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let ctx = Pulse::initiate();
        ctx.set_subscribe();
        ctx.set_sink_callback(tx, |x|{
            println!("{x}");
        });
        ctx.run();
        match rx.try_recv() {
            Ok(val) => {
                println!("{val}");
            }
            _ => {}
        }
    });

    ui::ui::ui_run();
    // loop {
    //     println!("{} this", rx.recv().unwrap());
    // }
}
