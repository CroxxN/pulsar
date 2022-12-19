use libpulse_binding as pulse;

use pulse::callbacks::ListResult;
use pulse::context::subscribe::InterestMaskSet;
// use pulse::context::introspect::SinkInfo;
use pulse::context::Context;
use pulse::mainloop::standard::Mainloop;
use pulse::proplist::Proplist;
// use pulse::stream::Direction;
// use pulse::volume::ChannelVolumes;


fn main() {
    let mut mainloop = Mainloop::new().expect("Failed to create mainloop");
    let mut context = Context::new_with_proplist(&mainloop, "pulsar", &Proplist::new().unwrap()).unwrap();

    context.connect(None, pulse::context::FlagSet::NOFLAGS, None).unwrap();
    context.set_state_callback(Some(Box::new(|| {
        println!("Connected");
    })));
    loop {
        if context.get_state() == pulse::context::State::Ready {
            context.set_state_callback(None);
            println!("Ready");
            break;
        }else {
            mainloop.iterate(false);
        }
    }

    let flags = InterestMaskSet::SINK_INPUT;

    context.subscribe(flags, |res|{
        println!("Subscribed: {:?}", res);
    });
    context.set_subscribe_callback(Some(Box::new(|_, _, index| {
        println!("{index}");
    })));
    
    context.introspect().get_sink_input_info_list(|list | {
        match list {
            ListResult::Item(value) => {
                println!("Sink {:?}", value.proplist);
            }
            _ => {println!("No sink found")}
        }
    });

    mainloop.run().unwrap();
}
