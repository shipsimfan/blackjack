use crate::model::{BlackjackTable, GameState, PlayerState};

impl BlackjackTable {
    /// Figure out the current state of the blackjack game, returning `true` if a new hand must be dealt
    pub(super) fn change_state(&mut self) -> bool {
        if self.state.is_round_active() {
            todo!("Handle round active");
        }

        let mut players = 0;
        let mut humans = 0;
        let mut bets_placed = 0;
        for player in self.sitting_players() {
            if player.state() == PlayerState::NotPlaying {
                continue;
            }

            players += 1;
            if !player.ai() {
                humans += 1;
            }

            if player.state() == PlayerState::PlayingThisRound {
                bets_placed += 1;
            }
        }

        if players >= self.min_players.get() && humans >= self.min_humans {
            if players == bets_placed {
                todo!("Figure out first player");
                true
            } else {
                self.state = GameState::WaitingForBets;
                false
            }
        } else {
            self.state = GameState::WaitingForPlayers;
            false
        }
    }
}
