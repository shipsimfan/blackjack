use crate::model::Shoe;

impl Shoe {
    /// Gets the current number of cards in the shoe and in the discard
    pub fn current_cards(&self) -> usize {
        self.cards.len() + self.discard.len()
    }

    /// Gets the total number of cards that make up this shoe
    pub fn size(&self) -> usize {
        self.size
    }
}
