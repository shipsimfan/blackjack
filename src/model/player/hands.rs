use crate::model::{Hand, Player, PlayerState};
use std::num::NonZeroU16;

impl Player {
    /// Add a hand to this player
    pub(crate) fn add_hand(&mut self, bet: NonZeroU16) {
        if self.state != PlayerState::PlayingThisRound {
            self.hands.truncate(0);
        }

        self.hands.push(Hand::new(Some(bet)));
    }
}
