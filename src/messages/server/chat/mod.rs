use std::borrow::Cow;

mod generate;
mod new;
mod parse;
mod to_static;

/// A chat message sent by a client, echoed by the server
#[derive(Debug, Clone)]
pub struct ChatServerMessage<'a> {
    /// The client that sent the message
    pub client: u8,

    /// The chat message
    pub message: Cow<'a, str>,
}
