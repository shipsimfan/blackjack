use crate::messages::{ChatClientMessage, ClientMessage};
use std::borrow::Cow;

impl<'a> ChatClientMessage<'a> {
    /// Creates a new [`ChatClientMessage`]
    pub fn new<S: Into<Cow<'a, str>>>(message: S) -> ClientMessage<'a> {
        ClientMessage::Chat(ChatClientMessage {
            message: message.into(),
        })
    }
}
