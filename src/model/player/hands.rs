use crate::model::{Hand, Player, PlayerState, Shoe};
use std::num::NonZeroU16;

impl Player {
    /// Add a hand to this player
    pub(crate) fn add_hand(&mut self, bet: NonZeroU16, shoe: Option<&mut Shoe>) {
        if self.state != PlayerState::PlayingThisRound {
            self.clear_hands(shoe);
            self.hands.truncate(0);
        }

        self.hands.push(Hand::new(Some(bet)));
    }

    /// Clears the hands and discards any cards to `shoe`
    pub fn clear_hands(&mut self, mut shoe: Option<&mut Shoe>) {
        self.last_round_earnings = None;
        for hand in &mut self.hands {
            hand.clear(&mut shoe);
        }
    }
}
