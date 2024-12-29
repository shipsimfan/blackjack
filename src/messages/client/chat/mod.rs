use std::borrow::Cow;

mod generate;
mod new;
mod parse;
mod to_static;

/// A chat message sent by a client
#[derive(Debug, Clone)]
pub struct ChatClientMessage<'a> {
    /// The chat message
    pub message: Cow<'a, str>,
}
