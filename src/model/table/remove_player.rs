use crate::{messages::ServerMessage, model::BlackjackTable};

impl BlackjackTable {
    /// Remove a player from the table at `id`, returning if a new hand was dealt and if the shoe was shuffled
    pub(super) fn remove_player<'a>(&mut self, id: usize) -> Option<(ServerMessage<'a>, bool)> {
        let mut message = None;
        if let Some(player) = &mut self.players[id] {
            player.clear_hands(self.shoe.as_mut());
            if self.change_state() && self.shoe.is_some() {
                message = Some(self.deal(None));
            }
        }

        self.players[id] = None;
        message
    }
}
