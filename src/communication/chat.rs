use crate::event::EventDispatcher;

use super::Origin;

pub type Chat = EventDispatcher<ChatMessage>;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub origin: Origin,
    pub message: String,
}