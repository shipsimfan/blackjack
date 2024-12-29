use crate::messages::{ChatServerMessage, ServerMessage};
use std::borrow::Cow;

impl<'a> ChatServerMessage<'a> {
    /// Creates a new [`ChatServerMessage`]
    pub fn new<S: Into<Cow<'a, str>>>(client: u8, message: S) -> ServerMessage<'a> {
        ServerMessage::Chat(ChatServerMessage {
            client,
            message: message.into(),
        })
    }
}
