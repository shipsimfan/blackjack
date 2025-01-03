use crate::model::{Hand, Player, PlayerState, Shoe};
use std::num::NonZeroU16;

impl Player {
    /// Add a hand to this player
    pub(crate) fn add_hand(&mut self, bet: NonZeroU16, shoe: Option<&mut Shoe>) {
        if self.state != PlayerState::PlayingThisRound {
            self.clear_hands(shoe);
        }

        self.hands.push(Hand::new(Some(bet)));
    }

    /// Clears the hands and discards any cards to `shoe`
    pub(crate) fn clear_hands(&mut self, shoe: Option<&mut Shoe>) {
        if let Some(shoe) = shoe {
            for hand in &self.hands {
                for card in hand.cards() {
                    shoe.discard(*card);
                }
            }
        }

        self.hands.truncate(0);
    }
}
