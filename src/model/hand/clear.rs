use crate::model::{Hand, Shoe};

impl Hand {
    /// Clears the hand and discards any cards to `shoe`
    pub fn clear(&mut self, shoe: &mut Option<&mut Shoe>) {
        if let Some(shoe) = shoe {
            for card in &self.cards {
                shoe.discard(*card);
            }
        }

        self.cards.truncate(0);
    }
}
