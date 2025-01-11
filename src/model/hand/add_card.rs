use crate::model::{Card, Hand};

impl Hand {
    /// Add `card` to this hand
    pub(crate) fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Inserts `card` at the beginning of the hand
    pub(crate) fn push_card_front(&mut self, card: Card) {
        self.cards.insert(0, card);
    }
}
