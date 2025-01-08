use super::GameState;
use crate::model::{BlackjackTable, PlayerState};

impl BlackjackTable {
    /// Ends the current round, preparing for the next
    pub(super) fn end_round(&mut self) {
        // TODO: Perform dealer play and end-round payouts

        // Reset player states
        for i in 0..self.players.len() {
            let player = match &mut self.players[i] {
                Some(player) => player,
                None => continue,
            };

            if player.state() == PlayerState::PlayingThisRound {
                player.set_state(PlayerState::PlayingNextRound);
            }
        }

        // Reset game state
        self.state = GameState::WaitingForPlayers;
        self.change_state(false);
    }
}
