use crate::model::{BlackjackTable, Player};

impl BlackjackTable {
    /// Gets the player with `id`
    pub fn player(&self, id: usize) -> &Player {
        self.players[id].as_ref().unwrap()
    }

    /// Gets the maximum bet a player can place at this table
    pub fn max_bet(&self) -> u16 {
        self.max_bet.get()
    }

    /// Gets the minimum bet a player can place at this table
    pub fn min_bet(&self) -> u16 {
        self.min_bet.get()
    }
}
