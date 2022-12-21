#![allow(dead_code)]
mod ui;

use libpulse_binding as pulse;

use pulse::callbacks::ListResult;
use pulse::context::subscribe::InterestMaskSet;
// use pulse::context::introspect::SinkInfo;
use pulse::context::Context;
use pulse::mainloop::standard::Mainloop;
use pulse::proplist::Proplist;
// use pulse::stream::Direction;
// use pulse::volume::ChannelVolumes;

struct Pulse {
    mainloop: Mainloop,
    context: Context,
}

impl Pulse {
    fn initiate() -> Pulse {
        let mut mainloop = Mainloop::new().expect("Failed to create mainloop");
        let mut context = Context::new_with_proplist(
            &mainloop,
            "pulsar",
            &Proplist::new().expect("Failed to create the proplist"),
        )
        .expect("Failed to create the context");

        context
            .connect(None, pulse::context::FlagSet::NOFLAGS, None)
            .unwrap();
        context.set_state_callback(Some(Box::new(|| {
            println!("Connected to pulse backend");
        })));
        loop {
            if context.get_state() == pulse::context::State::Ready {
                context.set_state_callback(None);
                println!("Ready");
                break;
            } else {
                mainloop.iterate(false);
            }
        }
        Pulse { mainloop, context }
    }
    fn set_subscribe(&mut self) {
        let flags = InterestMaskSet::SINK_INPUT;

        self.context.subscribe(flags, |res| {
            println!("Subscription: {:?}", res);
        });
    }
    fn run(&mut self) {
        self.mainloop.run().expect("Unable to run the mainloop");
    }
    fn get_sink_list(&mut self) {
        self.context
            .set_subscribe_callback(Some(Box::new(|_x, _y, index| {
                // ctx.introspect()
                //     .get_sink_info_by_index(index, |list| match list {
                //         ListResult::Item(value) => {
                //             println!("Sink {:?}", value.proplist);
                //         }
                //         _ => {
                //             println!("No sink found")
                //         }
                //     });
                // println!("{index}");
            })));
    }
}
fn main() {
    let mut ctx = Pulse::initiate();
    ctx.run();
    // context
    //     .introspect()
    //     .get_sink_input_info_list();
}
