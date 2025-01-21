use crate::{
    messages::{EndRoundServerMessage, ServerMessage},
    model::{BlackjackTable, GameState, HandleMessageResult, PlayerState},
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
                self.remove_player(disconnected.id as _)
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
                self.change_state(false, false)
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
                self.change_state(false, false)
            }
            ServerMessage::Deal(deal) => {
                self.dealer_hand.clear(&mut None);
                for player in &mut self.players {
                    if let Some(player) = player {
                        player.clear_hands(None);
                    }
                }

                if let Some(dealer_down) = deal.dealer_down_card {
                    self.dealer_hand.add_card(dealer_down);
                    self.dealer_hand.set_hidden_card(false);
                } else {
                    self.dealer_hand.set_hidden_card(true);
                }
                self.dealer_hand.add_card(deal.dealer_up_card);

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

                    return HandleMessageResult::EndRound(
                        EndRoundServerMessage::new(None, Vec::new()),
                        None,
                    );
                }

                // Check for player blackjack
                let payout = self.blackjack_payout;
                for player in self.sitting_players_mut() {
                    let mut amount = 0;
                    for hand in player.hands() {
                        if hand.cards().len() == 2 && hand.value().as_u8() == 21 {
                            amount += payout.apply(hand.bet().unwrap().get());
                        }
                    }

                    if amount != 0 {
                        player.payout(amount as _);
                    }
                }

                self.change_state(true, false)
            }
            ServerMessage::EndRound(end_round) => {
                if let Some(dealer_card) = end_round.dealer_card {
                    self.dealer_hand.set_hidden_card(false);
                    if self.dealer_hand.cards().len() != 2 {
                        self.dealer_hand.push_card_front(dealer_card);
                    }

                    for card in end_round.dealer_play {
                        self.dealer_hand.add_card(card);
                    }

                    let dealer_value = self.dealer_hand.value().as_u8();
                    for player in self.sitting_players_mut() {
                        if player.state() != PlayerState::PlayingThisRound {
                            continue;
                        }

                        let mut amount = 0;
                        for hand in player.hands() {
                            if hand.value().is_bust() {
                                continue;
                            }

                            if hand.value().as_u8() == 21 && hand.cards().len() == 2 {
                                continue;
                            }

                            if dealer_value > 21 || hand.value().as_u8() > dealer_value {
                                amount += hand.bet().unwrap().get() as i32;
                            } else if hand.value().as_u8() < dealer_value {
                                amount -= hand.bet().unwrap().get() as i32;
                            }
                        }

                        player.payout(amount);
                    }
                }

                // Reset player states
                for i in 0..self.players.len() {
                    let player = match &mut self.players[i] {
                        Some(player) => player,
                        None => continue,
                    };

                    if player.state() == PlayerState::PlayingThisRound {
                        player.set_state(PlayerState::PlayingNextRound, self.shoe.as_mut());
                    }
                }

                // Reset game state
                self.state = GameState::WaitingForPlayers;
                self.change_state(false, false)
            }
            ServerMessage::Hit(hit) => {
                let (player, hand) = match self.current_hand() {
                    Some(player) => player,
                    None => return HandleMessageResult::NoChange,
                };

                let player = self.player_mut(player);
                player.hands_mut()[hand].add_card(hit.card);
                if player.hands()[hand].value().as_u8() >= 21 {
                    self.change_state(false, false)
                } else {
                    HandleMessageResult::Change
                }
            }
            ServerMessage::Stand(_) => self.change_state(false, true),

            ServerMessage::Error(_)
            | ServerMessage::GameState(_)
            | ServerMessage::Hello(_)
            | ServerMessage::Chat(_)
            | ServerMessage::Shuffle(_) => HandleMessageResult::NoChange,
        }
    }
}
