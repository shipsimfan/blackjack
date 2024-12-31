use crate::model::{Card, Shoe};

impl Shoe {
    /// Draw a card from the shoe
    pub fn draw(&mut self) -> Card {
        if let Some(card) = self.cards.pop() {
            if let Some(cut_card) = self.cut_card {
                if self.cards.len() <= cut_card.get() {
                    self.cut_card = None;
                }
            }

            return card;
        }

        assert!(self.cut_card.is_none());

        self.shuffle(false);
        self.draw()
    }
}
