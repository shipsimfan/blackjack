use crate::model::{Card, Hand};

impl Hand {
    /// Add `card` to this hand
    pub(crate) fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}
