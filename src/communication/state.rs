use crate::event::EventDispatcher;

type State = EventDispatcher<StateUpdate>;

#[derive(Debug, Clone)]
pub struct StateUpdate {
    pub state: HostState,
}

#[derive(Debug, Clone)]
pub enum HostState {
    Starting,
    Running,
    Stopping,
    Stopped,
}