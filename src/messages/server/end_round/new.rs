use crate::{
    messages::{EndRoundServerMessage, ServerMessage},
    model::Card,
};

impl EndRoundServerMessage {
    /// Creates a new [`EndRoundServerMessage`]
    pub fn new<'a>(dealer_card: Option<Card>) -> ServerMessage<'a> {
        ServerMessage::EndRound(EndRoundServerMessage { dealer_card })
    }
}
