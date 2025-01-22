use crate::{
    messages::{EndRoundServerMessage, ShuffleServerMessage},
    model::{BlackjackTable, GameState, Hand, HandValue, HandleMessageResult, PlayerState},
};

impl BlackjackTable {
    /// Figure out the current state of the blackjack game
    pub(super) fn change_state<'a>(
        &mut self,
        round_start: bool,
        stand: bool,
    ) -> HandleMessageResult<'a> {
        if round_start || self.state.is_round_active() {
            if !stand && !round_start {
                let (current_player, current_hand) = self.current_hand().unwrap();

                // Check if player is still in the game
                if let Some(player) = &mut self.players[current_player] {
                    let hand = &player.hands()[current_hand];

                    // If the player is, see if they've reached >= 21 to move on to next hand
                    if hand.value().as_u8() < 21 {
                        return HandleMessageResult::Change;
                    }

                    // Handle busting
                    if hand.value().is_bust() {
                        player.payout(-(hand.bet().unwrap().get() as i32));
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
                        let mut dealer_play = Vec::new();
                        let mut shuffled = false;

                        let mut should_dealer_play = false;
                        for player in self.sitting_players() {
                            if player.state() != PlayerState::PlayingThisRound {
                                continue;
                            }

                            for hand in player.hands() {
                                if hand.value().as_u8() < 21
                                    || (hand.value().as_u8() == 21 && hand.cards().len() > 2)
                                {
                                    should_dealer_play = true;
                                    break;
                                }
                            }

                            if should_dealer_play {
                                break;
                            }
                        }

                        if should_dealer_play {
                            if let Some(shoe) = self.shoe.as_mut() {
                                let mut temp_hand = Hand::new(None);
                                for card in self.dealer_hand.cards() {
                                    temp_hand.add_card(*card);
                                }

                                while temp_hand.value().as_u8() <= 17 {
                                    match temp_hand.value() {
                                        HandValue::Hard(17) | HandValue::NoAce(17) => break,
                                        HandValue::Soft(17) => {
                                            if !self.hit_soft_17 {
                                                break;
                                            }
                                        }
                                        _ => {}
                                    }

                                    let (new_card, shuffle) = shoe.draw();
                                    shuffled |= shuffle;
                                    temp_hand.add_card(new_card);
                                    dealer_play.push(new_card);
                                }
                            }
                        }

                        HandleMessageResult::EndRound(
                            EndRoundServerMessage::new(
                                Some(self.dealer_hand.cards()[0]),
                                dealer_play,
                            ),
                            if shuffled {
                                Some(ShuffleServerMessage::new())
                            } else {
                                None
                            },
                        )
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
