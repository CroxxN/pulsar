#![allow(dead_code)]

mod error;
use error::PulseCoreError;
use libpulse_binding as pulse;

use pulse::callbacks::ListResult;
use pulse::context::subscribe::{Facility, InterestMaskSet, Operation};
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
    fn init() -> Result<(), PulseCoreError> {
        let mut mainloop = Mainloop::new().expect("Failed to create mainloop");
        let mut context =
            Context::new_with_proplist(&mainloop, "pulsar", &Proplist::new().unwrap()).unwrap();

        context.connect(None, pulse::context::FlagSet::NOFLAGS, None)?;
        context.set_state_callback(Some(Box::new(|| {
            println!("Connected");
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
        Ok(())
    }
    fn run(&mut self) {
        self.mainloop.run().unwrap();
    }
    fn get_sink_list<F>(&mut self, mask: InterestMaskSet, callback: F)
    where
        F: FnMut(Option<Facility>, Option<Operation>, u32) + 'static,
    {
        self.context.subscribe(mask, |res| {
            if !res {
                return;
            }
        });
        self.context
            .set_subscribe_callback(Some(Box::new(callback)));
        self.context
            .introspect()
            .get_sink_input_info_list(|list| match list {
                ListResult::Item(value) => {
                    println!("Sink {:?}", value.proplist);
                }
                _ => {
                    println!("No sink found")
                }
            });
    }
    fn subscribe_volume_change<F>(&mut self) {}
}
