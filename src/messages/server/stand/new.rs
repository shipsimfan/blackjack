use crate::messages::{ServerMessage, StandServerMessage};

impl StandServerMessage {
    /// Creates a new [`StandServerMessage`]
    pub fn new<'a>() -> ServerMessage<'a> {
        ServerMessage::Stand(StandServerMessage {})
    }
}
