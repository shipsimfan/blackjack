use crate::messages::{ClientMessage, PlayNextRoundClientMessage};

impl PlayNextRoundClientMessage {
    /// Creates a new [`PlayNextRoundClientMessage`]
    pub fn new<'a>(play_next_round: bool) -> ClientMessage<'a> {
        ClientMessage::PlayNextRound(PlayNextRoundClientMessage { play_next_round })
    }
}
