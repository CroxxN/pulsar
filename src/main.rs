#![allow(dead_code)]
mod ui;

use libpulse_binding as pulse;
use pulse::callbacks::ListResult;
use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};
// use pulse::context::introspect::SinkInfo;
use pulse::context;
use pulse::mainloop::standard::Mainloop;
use pulse::proplist::Proplist;

struct Pulse {
    mainloop: Rc<RefCell<Mainloop>>,
    context: Rc<RefCell<context::Context>>,
}

impl Pulse {
    fn initiate() -> Pulse {
        let mainloop = Rc::new(RefCell::new(
            Mainloop::new().expect("Failed to create mainloop"),
        ));
        let context = Rc::new(RefCell::new(
            context::Context::new_with_proplist(
                mainloop.borrow().deref(),
                "pulsar",
                &Proplist::new().expect("Failed to create the proplist"),
            )
            .expect("Failed to create the context"),
        ));

        context
            .borrow_mut()
            .connect(None, pulse::context::FlagSet::NOFLAGS, None)
            .unwrap();
        context.borrow_mut().set_state_callback(Some(Box::new(|| {
            println!("Connected to pulse backend");
        })));
        loop {
            if context.borrow_mut().get_state() == pulse::context::State::Ready {
                context.borrow_mut().set_state_callback(None);
                println!("Ready");
                break;
            } else {
                mainloop.borrow_mut().iterate(false);
            }
        }
        Pulse { mainloop, context }
    }
    fn set_subscribe(&self) {
        let flags = context::subscribe::InterestMaskSet::SINK_INPUT;

        self.context.borrow_mut().subscribe(flags, |res| {
            println!("Subscription: {:?}", res);
        });
    }
    fn run(&self) {
        self.mainloop
            .borrow_mut()
            .run()
            .expect("Unable to run the mainloop");
    }
    fn get_sink_list(&self) {
        let ctx = Rc::clone(&self.context);
        self.context
            .borrow_mut()
            .set_subscribe_callback(Some(Box::new(move |_x, operation, index| {
                if let Some(op) = operation {
                    match op {
                        // TODO: Implement on change
                        context::subscribe::Operation::New
                        | context::subscribe::Operation::Removed => {
                            ctx.borrow_mut()
                                .introspect()
                                .get_sink_input_info(index, |list| match list {
                                    ListResult::Item(value) => {
                                        println!("Sink {:?}", value.proplist);
                                    }
                                    _ => {
                                        println!("No sink found")
                                    }
                                });
                        }
                        _ => {}
                    }
                }
            })));
    }
}
fn main() {
    let ctx = Pulse::initiate();
    ctx.set_subscribe();
    ctx.get_sink_list();
    ctx.run();
}
