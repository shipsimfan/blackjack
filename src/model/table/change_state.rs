use crate::model::{BlackjackTable, GameState, PlayerState};

impl BlackjackTable {
    pub(super) fn change_state(&mut self) {
        if self.state.is_round_active() {
            return;
        }

        let mut players = 0;
        let mut humans = 0;
        for player in self.sitting_players() {
            if player.state() == PlayerState::NotPlaying {
                continue;
            }

            players += 1;
            if !player.ai() {
                humans += 1;
            }
        }

        self.state = if players >= self.min_players.get() && humans >= self.min_humans {
            GameState::WaitingForBets
        } else {
            GameState::WaitingForPlayers
        };
    }
}
