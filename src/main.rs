use pulsectl::controllers::{AppControl, SinkController};

fn main() {
    let mut handler = SinkController::create().unwrap();
    let devices = handler.list_applications().unwrap();
    for device in devices {
        let idx = handler.get_app_by_index(device.index).unwrap();
        println!("{:?}", idx.mute);
    }
}
