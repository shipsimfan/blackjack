use crate::model::{BlackjackTable, Player};

impl BlackjackTable {
    /// Add a player to the table at `id`
    pub(super) fn add_player(&mut self, id: usize, player: Player) {
        assert!(self.players[id].is_none());
        self.players[id] = Some(player);
    }
}
