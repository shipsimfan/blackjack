use crate::model::{Card, Hand, HandValue};
use std::num::NonZeroU16;

impl Hand {
    /// Gets the bet for this hand, or [`None`] if it is the dealer's hand
    pub fn bet(&self) -> Option<NonZeroU16> {
        self.bet
    }

    /// Gets the cards in this hand
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }

    /// Computes the cards for this hand
    pub fn value(&self) -> HandValue {
        HandValue::compute(&self.cards)
    }

    /// Does this hand contain a hidden card?
    pub fn hidden_card(&self) -> bool {
        self.hidden_card
    }
}
