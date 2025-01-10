use crate::model::{BlackjackTable, HandleMessageResult};

impl BlackjackTable {
    /// Remove a player from the table at `id`, returning if a new hand was dealt and if the shoe was shuffled
    pub(super) fn remove_player<'a>(&mut self, id: usize) -> HandleMessageResult<'a> {
        let mut player = match self.players[id].take() {
            Some(player) => player,
            None => return HandleMessageResult::Change,
        };
        player.clear_hands(self.shoe.as_mut());
        self.change_state(false)
    }
}
