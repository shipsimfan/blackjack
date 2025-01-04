use crate::model::BlackjackTable;

impl BlackjackTable {
    /// Remove a player from the table at `id`
    pub(super) fn remove_player(&mut self, id: usize) {
        if let Some(player) = &mut self.players[id] {
            player.clear_hands(self.shoe.as_mut());

            todo!("Check if game state must be changed or hand must be dealt");
        }

        self.players[id] = None;
    }
}
