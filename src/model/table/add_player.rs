use crate::model::{BlackjackTable, Player};

impl BlackjackTable {
    /// Add a player to the table at `id`
    pub fn add_player(&mut self, player: Player, id: usize) {
        assert!(self.players[id].is_none());
        self.players[id] = Some(player);
    }
}
