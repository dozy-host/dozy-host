
use std::pin::Pin;

use futures::{future::Shared, FutureExt};

pub struct HostEvent {
    pub origin: MessageOrigin,
    pub kind: HostEventType,
}

type SharedFuture<T> = Shared<Pin<Box<dyn FutureExt<Output = T>>>>;

pub enum HostEventType {
    Start(SharedFuture<Result<(), ()>>),
    Stop(SharedFuture<Result<(), ()>>),
    Restart,
    Pause,
    Resume,
    Kill,
    Status,
}

pub enum MessageOrigin {
    Server,
    Website,
    Discord,
}
