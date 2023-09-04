use std::rc::Rc;

pub struct EventDispatcher<O: Clone> {
    peripherals: Vec<Rc<dyn EventHandler<Event = O>>>
}

impl<O> EventDispatcher<O> where O: Clone {
    pub fn register(&mut self, peripheral: Rc<dyn EventHandler<Event = O>>) {
        self.peripherals.push(peripheral);
    }

    pub fn emit(&self, event: &O) {
        for peripheral in &self.peripherals {
            peripheral.event(event.to_owned());
        }
    }
}

pub trait EventHandler {
    type Event;
    fn event(&self, e: Self::Event);
}