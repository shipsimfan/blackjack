use crate::model::{BlackjackTable, GameState, PlayerState};

impl BlackjackTable {
    /// Figure out the current state of the blackjack game, returning `true` if a new hand must be dealt
    pub(super) fn change_state(&mut self) -> bool {
        if self.state.is_round_active() {
            let (current_player, current_hand) = self.current_hand().unwrap();

            // Check if player is still in the game
            if let Some(player) = &self.players[current_player] {
                // If the player is, see if they've reached >= 21 to move on to next hand
                if player.hands()[current_hand].value().as_u8() <= 21 {
                    return false;
                }
            }

            // If not, move on to the next player
            match self.next_hand() {
                Some((next_player, next_hand)) => {
                    self.state = GameState::WaitingForPlayer(next_player as _, next_hand as _);
                }
                None => {
                    // TODO: Handle ending round

                    self.state = GameState::WaitingForPlayers;
                    self.change_state();
                }
            }

            return false;
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
                if let Some((first_player, first_hand)) = self.next_hand() {
                    self.state = GameState::WaitingForPlayer(first_player as _, first_hand as _);
                }
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
