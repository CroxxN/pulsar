use libpulse_binding as pulse;
use pulse::callbacks::ListResult;
use pulse::context::introspect::SinkInputInfo;
use pulse::context::subscribe::{Facility, Operation};
use std::ops::Deref;
use std::{
    cell::RefCell,
    rc::Rc
};
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
    fn handle_callback<F>(index: u32, f:F) where F: Fn(ListResult<&SinkInputInfo>){
        let mut thing = String::new();
        // self.context.borrow()
        //     .introspect()
        //     .get_sink_input_info(index, f);
    }
    pub fn set_sink_callback<F>(&self, f:F) where F:Fn(Option<Facility>, Option<Operation>, u32) + 'static{
        
        self.context
            .borrow_mut()
            .set_subscribe_callback(Some(Box::new(f)));
    }

}
