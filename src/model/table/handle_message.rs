use crate::{
    messages::ServerMessage,
    model::{BlackjackTable, HandleMessageResult, PlayerState},
};

impl BlackjackTable {
    /// Handles `message`, returning true if something changed about the table
    pub fn handle_message<'a>(&mut self, message: ServerMessage) -> HandleMessageResult<'a> {
        match message {
            ServerMessage::ClientConnected(connected) => {
                self.add_player(connected.id as _, connected.unwrap());
                HandleMessageResult::Change
            }
            ServerMessage::ClientDisconnected(disconnected) => {
                return match self.remove_player(disconnected.id as _) {
                    Some((deal, shuffle)) => HandleMessageResult::Deal(deal, shuffle),
                    None => HandleMessageResult::Change,
                };
            }
            ServerMessage::PlayNextRound(play_next_round) => {
                let player = self.players[play_next_round.client as usize]
                    .as_mut()
                    .unwrap();
                if player.state() == PlayerState::PlayingThisRound
                    || player.state() == play_next_round.as_state()
                {
                    return HandleMessageResult::NoChange;
                }

                player.set_state(play_next_round.as_state(), self.shoe.as_mut());
                if self.change_state(false) && self.shoe.is_some() {
                    let (deal, shuffle) = self.deal(None);
                    return HandleMessageResult::Deal(deal, shuffle);
                }

                HandleMessageResult::Change
            }
            ServerMessage::PlaceBet(place_bet) => {
                if self.state.is_round_active()
                    || place_bet.bet < self.min_bet
                    || place_bet.bet > self.max_bet
                {
                    return HandleMessageResult::NoChange;
                }

                let max_hands = self.max_hands.get();

                let player = self.players[place_bet.client as usize].as_mut().unwrap();
                if player.state() == PlayerState::NotPlaying
                    || (player.state() == PlayerState::PlayingThisRound
                        && player.hands().len() >= max_hands as _)
                {
                    return HandleMessageResult::NoChange;
                }

                player.add_hand(place_bet.bet, self.shoe.as_mut());
                player.set_state(PlayerState::PlayingThisRound, self.shoe.as_mut());
                if self.change_state(false) && self.shoe.is_some() {
                    let (deal, shuffle) = self.deal(None);
                    return HandleMessageResult::Deal(deal, shuffle);
                }

                HandleMessageResult::Change
            }
            ServerMessage::Deal(deal) => {
                self.dealer_hand.clear(&mut None);
                for player in &mut self.players {
                    if let Some(player) = player {
                        player.clear_hands(None);
                    }
                }

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

                    for hand in player.hands_mut() {
                        hand.add_card(deal.hands[i].0);
                        hand.add_card(deal.hands[i].1);
                        i += 1;
                    }
                }

                // Check for dealer blackjack
                if self.dealer_hand.cards().len() == 2 && self.dealer_hand.value().as_u8() == 21 {
                    for player in self.sitting_players_mut() {
                        if player.state() != PlayerState::PlayingThisRound {
                            continue;
                        }

                        let mut amount = 0;
                        for hand in player.hands() {
                            if hand.value().as_u8() < 21 {
                                amount -= hand.bet().unwrap().get() as i32;
                            }
                        }
                        player.payout(amount);
                    }

                    if self.end_round(false) && self.shoe.is_some() {
                        let (deal, shuffle) = self.deal(None);
                        return HandleMessageResult::Deal(deal, shuffle);
                    }
                    return HandleMessageResult::Change;
                }

                // Check for player blackjack
                for player in self.sitting_players_mut() {
                    let mut amount = 0;
                    for hand in player.hands() {
                        if hand.cards().len() == 2 && hand.value().as_u8() == 21 {
                            amount += (3 * hand.bet().unwrap().get() as u32) / 2;
                        }
                    }

                    if amount != 0 {
                        player.payout(amount as _);
                    }
                }

                self.change_state(true);

                HandleMessageResult::Change
            }

            ServerMessage::Error(_)
            | ServerMessage::GameState(_)
            | ServerMessage::Hello(_)
            | ServerMessage::Chat(_)
            | ServerMessage::Shuffle(_) => HandleMessageResult::NoChange,
        }
    }
}
