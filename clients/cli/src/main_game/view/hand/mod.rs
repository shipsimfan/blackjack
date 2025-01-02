use blackjack::model::Card;
use std::num::NonZeroU16;

mod new;
mod render;

/// The view of a hand
pub struct HandView {
    /// The current bet placed for the hand
    bet: Option<NonZeroU16>,

    /// The cards currently rendered in the hand
    cards: Vec<Card>,

    /// The y-level to render this hand at
    y: usize,

    /// The width available to render in
    width: usize,

    /// The maximum length a bet can bet
    max_bet_length: usize,
}
