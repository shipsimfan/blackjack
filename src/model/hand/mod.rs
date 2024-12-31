use std::num::NonZeroU16;

mod generate;
mod get;
mod new;
mod parse;

/// A single hand of blackjack
#[derive(Debug, Clone)]
pub struct Hand {
    /// The current bet of the hand, or [`None`] if this is the dealer's hand
    bet: Option<NonZeroU16>,
}
