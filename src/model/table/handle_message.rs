use crate::{
    messages::ServerMessage,
    model::{BlackjackTable, Hand, PlayerState},
};

impl BlackjackTable {
    // TODO: Change return type to `HandleMessageResult` enum to allow returning a deal server message

    /// Handles `message`, returning true if something changed about the table
    pub fn handle_message(&mut self, message: ServerMessage) -> bool {
        match message {
            ServerMessage::ClientConnected(connected) => {
                self.add_player(connected.id as _, connected.unwrap())
            }
            ServerMessage::ClientDisconnected(disconnected) => {
                if let Some((deal, shuffle)) = self.remove_player(disconnected.id as _) {
                    todo!()
                }
            }
            ServerMessage::PlayNextRound(play_next_round) => {
                let player = self.player_mut(play_next_round.client as _);
                if player.state() == PlayerState::PlayingThisRound {
                    return false;
                }

                player.set_state(play_next_round.as_state());
                if self.change_state() {
                    todo!();
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

                let player = self.players[place_bet.client as usize].as_mut().unwrap();
                if player.state() == PlayerState::NotPlaying
                    || (player.state() == PlayerState::PlayingThisRound
                        && player.hands().len() >= max_hands as _)
                {
                    return false;
                }

                player.add_hand(place_bet.bet, self.shoe.as_mut());
                player.set_state(PlayerState::PlayingThisRound);
                if self.change_state() {
                    todo!()
                }
            }
            ServerMessage::Deal(deal) => {
                self.dealer_hand = Hand::new(None);
                self.dealer_hand.add_card(deal.dealer_up_card);
                if let Some(dealer_down) = deal.dealer_down_card {
                    self.dealer_hand.add_card(dealer_down);
                }

                let mut i = 0;
                for player in &mut self.players {
                    let player = match player {
                        Some(player) => player,
                        None => continue,
                    };

                    if player.state() != PlayerState::PlayingThisRound {
                        player.clear_hands(self.shoe.as_mut());
                        continue;
                    }

                    for hand in player.hands_mut() {
                        hand.add_card(deal.hands[i].0);
                        hand.add_card(deal.hands[i].1);
                        i += 1;
                    }
                }
            }

            ServerMessage::Error(_)
            | ServerMessage::GameState(_)
            | ServerMessage::Hello(_)
            | ServerMessage::Chat(_) => return false,
        }

        true
    }
}
