use crate::{
    messages::Username,
    model::{Player, PlayerState},
};

impl Player {
    /// Gets the username of the player
    pub fn username(&self) -> &Username {
        &self.username
    }

    /// Is this player an AI?
    pub fn ai(&self) -> bool {
        self.ai
    }

    /// Gets the current state of this player
    pub fn state(&self) -> PlayerState {
        self.state
    }
}
