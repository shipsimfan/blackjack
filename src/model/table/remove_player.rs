use crate::{messages::ServerMessage, model::BlackjackTable};

impl BlackjackTable {
    /// Remove a player from the table at `id`, returning if a new hand was dealt and if the shoe was shuffled
    pub(super) fn remove_player<'a>(
        &mut self,
        id: usize,
    ) -> Option<(ServerMessage<'a>, Option<ServerMessage<'a>>)> {
        let mut player = self.players[id].take()?;
        player.clear_hands(self.shoe.as_mut());
        if self.change_state(false) && self.shoe.is_some() {
            return Some(self.deal(None));
        }

        None
    }
}
