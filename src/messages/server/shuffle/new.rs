use crate::messages::{ServerMessage, ShuffleServerMessage};

impl ShuffleServerMessage {
    /// Creates a new [`ShuffleServerMessage`]
    pub fn new<'a>() -> ServerMessage<'a> {
        ServerMessage::Shuffle(ShuffleServerMessage {})
    }
}
