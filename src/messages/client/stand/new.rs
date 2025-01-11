use crate::messages::{ClientMessage, StandClientMessage};

impl StandClientMessage {
    /// Creates a new [`StandClientMessage`]
    pub fn new<'a>() -> ClientMessage<'a> {
        ClientMessage::Stand(StandClientMessage {})
    }
}
