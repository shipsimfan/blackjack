use crate::model::Hand;
use std::num::NonZeroU16;

impl Hand {
    /// Gets the bet for this hand, or [`None`] if it is the dealer's hand
    pub fn bet(&self) -> Option<NonZeroU16> {
        self.bet
    }
}
