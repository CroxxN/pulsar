
#![allow(dead_code)]
mod pulse;
pub use pulse::Pulse;
use std::{sync::mpsc::channel, thread};

fn main() {
    let (tx, rx) = channel::<String>();
    thread::spawn(move || {
        let ctx = Pulse::initiate();
        ctx.set_subscribe();
        ctx.set_sink_callback(tx);
        ctx.run();
    });

    while let Ok(ret) = rx.recv() {
        println!("{ret}");
    }

    // ui::ui::ui_run(rx);
    // loop {
    //     println!("{} this", rx.recv().unwrap());
    // }
}
