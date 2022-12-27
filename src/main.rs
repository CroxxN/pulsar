#![allow(dead_code)]
mod ui;
mod pulse;
pub use pulse::Pulse;
use std::thread;

fn main() {
    thread::spawn(||{
        ui::ui::ui_run();
    });
    let ctx = Pulse::initiate();
    ctx.set_subscribe();
    ctx.set_sink_callback(|_x, _y, z|{
        println!("{z}");
    });
    ctx.run();
}
