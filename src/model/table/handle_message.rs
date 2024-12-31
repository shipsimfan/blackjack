use crate::{
    messages::ServerMessage,
    model::{BlackjackTable, GameState, PlayerState},
};

impl BlackjackTable {
    /// Handles `message`, returning true if something changed about the table
    pub fn handle_message(&mut self, message: ServerMessage) -> bool {
        match message {
            ServerMessage::ClientConnected(connected) => {
                self.add_player(connected.id as _, connected.unwrap())
            }
            ServerMessage::ClientDisconnected(disconnected) => {
                self.remove_player(disconnected.id as _)
            }
            ServerMessage::PlayNextRound(play_next_round) => {
                let player = self.player_mut(play_next_round.client as _);
                if player.state() == PlayerState::PlayingThisRound {
                    return false;
                }

                player.set_state(play_next_round.as_state());

                if !self.state.is_round_active() {
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
            ServerMessage::PlaceBet(place_bet) => {
                if self.state.is_round_active()
                    || place_bet.bet < self.min_bet
                    || place_bet.bet > self.max_bet
                {
                    return false;
                }

                let max_hands = self.max_hands.get();

                let player = self.player_mut(place_bet.client as _);
                if player.state() == PlayerState::NotPlaying
                    || (player.state() == PlayerState::PlayingThisRound
                        && player.hands().len() >= max_hands as _)
                {
                    return false;
                }

                player.add_hand(place_bet.bet);
                player.set_state(PlayerState::PlayingThisRound);
            }

            ServerMessage::Error(_)
            | ServerMessage::GameState(_)
            | ServerMessage::Hello(_)
            | ServerMessage::Chat(_) => return false,
        }

        true
    }
}
