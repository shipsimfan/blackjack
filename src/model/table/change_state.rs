use crate::{
    messages::EndRoundServerMessage,
    model::{BlackjackTable, GameState, HandleMessageResult, PlayerState},
};

impl BlackjackTable {
    /// Figure out the current state of the blackjack game
    pub(super) fn change_state<'a>(&mut self, round_start: bool) -> HandleMessageResult<'a> {
        if round_start || self.state.is_round_active() {
            if !round_start {
                let (current_player, current_hand) = self.current_hand().unwrap();

                // Check if player is still in the game
                if let Some(player) = &self.players[current_player] {
                    // If the player is, see if they've reached >= 21 to move on to next hand
                    if player.hands()[current_hand].value().as_u8() < 21 {
                        return HandleMessageResult::Change;
                    }
                }
            }

            // If not, move on to the next player
            return match self.next_hand() {
                Some((next_player, next_hand)) => {
                    self.state = GameState::WaitingForPlayer(next_player as _, next_hand as _);
                    HandleMessageResult::Change
                }
                None => {
                    if self.dealer_hand.cards().len() == 2 {
                        HandleMessageResult::EndRound(EndRoundServerMessage::new(Some(
                            self.dealer_hand.cards()[0],
                        )))
                    } else {
                        HandleMessageResult::Change
                    }
                }
            };
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
                if self.shoe.is_some() {
                    let (deal, shuffle) = self.deal(None);
                    HandleMessageResult::Deal(deal, shuffle)
                } else {
                    HandleMessageResult::Change
                }
            } else {
                self.state = GameState::WaitingForBets;
                HandleMessageResult::Change
            }
        } else {
            self.state = GameState::WaitingForPlayers;
            HandleMessageResult::Change
        }
    }
}
