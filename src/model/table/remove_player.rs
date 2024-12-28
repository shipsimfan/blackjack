use crate::model::BlackjackTable;

impl BlackjackTable {
    /// Remove a player from the table at `id`
    pub(super) fn remove_player(&mut self, id: usize) {
        self.players[id] = None;
    }
}
