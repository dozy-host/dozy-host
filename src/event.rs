use std::sync::Arc;

pub struct EventDispatcher<O: Clone> {
    peripherals: Vec<Arc<dyn EventHandler<O>>>
}

impl<O> EventDispatcher<O> where O: Clone {
    pub fn register(&mut self, peripheral: Arc<dyn EventHandler<O>>) {
        self.peripherals.push(peripheral);
    }

    pub fn emit(&self, event: &O) {
        for peripheral in &self.peripherals {
            peripheral.event(event.to_owned());
        }
    }
}

pub trait EventHandler<O> {
    fn event(&self, e: O);
}