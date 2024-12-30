use crate::model::{Player, PlayerState};

impl Player {
    /// Set the current state of this player
    pub(crate) fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }
}
