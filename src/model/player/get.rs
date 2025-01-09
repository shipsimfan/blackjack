use crate::{
    messages::Username,
    model::{Hand, Player, PlayerState},
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

    /// Gets the hands this player currently has
    pub fn hands(&self) -> &[Hand] {
        &self.hands
    }

    /// Gets this players total earnings over their play session
    pub fn total_earnings(&self) -> i64 {
        self.total_earnings
    }

    /// Gets the players earnings from the previous round, if they played in it
    pub fn last_round_earnings(&self) -> Option<i32> {
        self.last_round_earnings
    }

    /// Gets the hands this player currently has mutably
    pub(crate) fn hands_mut(&mut self) -> &mut [Hand] {
        &mut self.hands
    }
}
