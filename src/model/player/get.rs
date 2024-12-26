use crate::{messages::Username, model::Player};

impl Player {
    /// Gets the username of the player
    pub fn username(&self) -> &Username {
        &self.username
    }

    /// Is this player an AI?
    pub fn ai(&self) -> bool {
        self.ai
    }
}
