use crate::messages::{PlayNextRoundServerMessage, ServerMessage};

impl PlayNextRoundServerMessage {
    /// Creates a new [`PlayNextRoundServerMessage`]
    pub fn new<'a>(client: u8, play_next_round: bool) -> ServerMessage<'a> {
        ServerMessage::PlayNextRound(PlayNextRoundServerMessage {
            client,
            play_next_round,
        })
    }
}
