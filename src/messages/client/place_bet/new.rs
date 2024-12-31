use crate::messages::{ClientMessage, PlaceBetClientMessage};
use std::num::NonZeroU16;

impl PlaceBetClientMessage {
    /// Creates a new [`PlaceBetClientMessage`]
    pub fn new<'a>(bet: NonZeroU16) -> ClientMessage<'a> {
        ClientMessage::PlaceBet(PlaceBetClientMessage { bet })
    }
}
