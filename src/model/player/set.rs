use crate::model::{Player, PlayerState, Shoe};

impl Player {
    /// Set the current state of this player
    pub(crate) fn set_state(&mut self, state: PlayerState, shoe: Option<&mut Shoe>) {
        self.state = state;

        if state == PlayerState::NotPlaying {
            self.clear_hands(shoe);
            self.hands.truncate(0);
        }
    }
}
