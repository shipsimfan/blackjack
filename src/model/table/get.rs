use crate::model::{BlackjackTable, Player};

impl BlackjackTable {
    /// Gets the player with `id`
    pub fn player(&self, id: usize) -> &Player {
        self.players[id].as_ref().unwrap()
    }
}
