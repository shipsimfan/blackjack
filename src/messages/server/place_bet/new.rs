use crate::messages::{PlaceBetServerMessage, ServerMessage};
use std::num::NonZeroU16;

impl PlaceBetServerMessage {
    /// Creates a new [`PlaceBetServerMessage`]
    pub fn new<'a>(client: u8, bet: NonZeroU16) -> ServerMessage<'a> {
        ServerMessage::PlaceBet(PlaceBetServerMessage { client, bet })
    }
}
