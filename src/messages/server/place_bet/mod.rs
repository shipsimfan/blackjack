use std::num::NonZeroU16;

mod generate;
mod new;
mod parse;

/// A player has placed a bet
#[derive(Debug, Clone)]
pub struct PlaceBetServerMessage {
    /// The client that placed the bet
    pub client: u8,

    /// The money that was bet
    pub bet: NonZeroU16,
}
