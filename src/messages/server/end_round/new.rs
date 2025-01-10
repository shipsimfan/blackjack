use crate::messages::{EndRoundServerMessage, ServerMessage};

impl EndRoundServerMessage {
    /// Creates a new [`EndRoundServerMessage`]
    pub fn new<'a>(dealer_play: bool) -> ServerMessage<'a> {
        ServerMessage::EndRound(EndRoundServerMessage { dealer_play })
    }
}
