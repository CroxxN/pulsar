use libpulse_binding as pulse;
use pulse::callbacks::ListResult;
use pulse::context::subscribe::Operation;
use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};
// use pulse::context::introspect::SinkInfo;
use pulse::context;
use pulse::mainloop::standard::Mainloop;
use pulse::proplist::Proplist;

pub struct Pulse {
    mainloop: Rc<RefCell<Mainloop>>,
    context: Rc<RefCell<context::Context>>,
}

impl Pulse {
    pub fn initiate() -> Pulse {
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
    pub fn set_subscribe(&self) {
        let flags = context::subscribe::InterestMaskSet::SINK_INPUT;

        self.context.borrow_mut().subscribe(flags, |res| {
            println!("Subscription: {:?}", res);
        });
    }
    pub fn run(&self) {
        self.mainloop
            .borrow_mut()
            .run()
            .expect("Unable to run the mainloop");
    }
    fn handle_sink_change_callback(
        ctx: Rc<RefCell<pulse::context::Context>>,
        index: u32,
        tx: std::sync::mpsc::Sender<String>,
        f: fn(String) -> () 
    ) {
        println!("Handeling Pulse Change");
        ctx.borrow_mut()
            .introspect()
            .get_sink_input_info(index, move |res| match res {
                ListResult::Item(val) => {
                    println!("{val:?}");
                    //_ = tx
                        //.send(val.proplist.to_string().to_owned().unwrap_or_default());
                        f(val.proplist.to_string().to_owned().unwrap_or_default());
                }
                _ => {}
            });
    }
    pub fn set_sink_callback(&self, tx: std::sync::mpsc::Sender<String>, f: fn(String) -> ()) {
        let ctx = Rc::clone(&self.context);
        self.context
            .borrow_mut()
            .set_subscribe_callback(Some(Box::new(move |_fac, op, index| match op {
                Some(Operation::New | Operation::Removed) => {
                    let txc = tx.clone();
                    println!("{op:?}");
                    Self::handle_sink_change_callback(ctx.clone(), index, txc, f);
                    // let _ =
                }
                _ => {}
            })));
    }
}
