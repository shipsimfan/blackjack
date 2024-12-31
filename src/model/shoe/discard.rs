use crate::model::{Card, Shoe};

impl Shoe {
    /// Add a card to the discard pile
    pub fn discard(&mut self, card: Card) {
        self.discard.push(card);
        assert!(self.discard.len() + self.cards.len() <= self.size);
    }
}
