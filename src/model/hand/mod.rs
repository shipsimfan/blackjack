use crate::model::Card;
use std::num::NonZeroU16;

mod value;

mod generate;
mod get;
mod new;
mod parse;

pub use value::HandValue;

/// A single hand of blackjack
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    /// The current bet of the hand, or [`None`] if this is the dealer's hand
    bet: Option<NonZeroU16>,

    /// The cards that make up the hand
    cards: Vec<Card>,
}
