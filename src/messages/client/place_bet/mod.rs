use std::num::NonZeroU16;

mod generate;
mod new;
mod parse;

/// A client placed a bet for the next round
#[derive(Debug, Clone)]
pub struct PlaceBetClientMessage {
    /// The amount of money bet
    pub bet: NonZeroU16,
}
