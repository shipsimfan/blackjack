use crate::model::{BlackjackTable, Player};

impl BlackjackTable {
    /// Gets all player slots
    pub fn players(&self) -> &[Option<Player>] {
        &self.players
    }

    /// Gets only the players seated at the table
    pub fn sitting_players(&self) -> impl Iterator<Item = &Player> {
        self.players.iter().filter_map(Option::as_ref)
    }

    /// Gets the player with `id`
    pub fn player(&self, id: usize) -> &Player {
        self.players[id].as_ref().unwrap()
    }

    /// Gets the maximum number of allowed players at this table
    pub fn max_players(&self) -> usize {
        self.players.len()
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
