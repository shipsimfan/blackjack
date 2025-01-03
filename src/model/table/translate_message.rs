use crate::{
    messages::{ClientMessage, PlaceBetServerMessage, PlayNextRoundServerMessage, ServerMessage},
    model::{BlackjackTable, GameState, PlayerState},
};

impl BlackjackTable {
    /// Translates a [`ClientMessage`] into zero or more corresponding [`ServerMessage`]s
    pub fn translate_message<'a>(
        &mut self,
        client_id: usize,
        message: &'a ClientMessage,
    ) -> Vec<ServerMessage<'a>> {
        match message {
            ClientMessage::PlayNextRound(play_next_round) => {
                let player = self.player(client_id);
                if player.state() == PlayerState::PlayingThisRound
                    || player.state() == play_next_round.as_state()
                {
                    return Vec::new();
                }

                let play_next_round = PlayNextRoundServerMessage::new(
                    client_id as _,
                    play_next_round.play_next_round,
                );

                if self.state == GameState::WaitingForBets {
                    let mut deal = true;
                    for player in self.sitting_players() {
                        if player.state() == PlayerState::PlayingNextRound {
                            deal = false;
                            break;
                        }
                    }

                    if deal {
                        let (deal, shuffled) = self.deal();
                        return vec![play_next_round, deal];
                    }
                }

                vec![play_next_round]
            }
            ClientMessage::PlaceBet(place_bet) => {
                let player = self.player(client_id);
                if player.state() != PlayerState::PlayingNextRound
                    || self.state.is_round_active()
                    || place_bet.bet < self.min_bet
                    || place_bet.bet > self.max_bet
                {
                    return Vec::new();
                }

                let bet = PlaceBetServerMessage::new(client_id as _, place_bet.bet);
                if self.state != GameState::WaitingForBets {
                    return vec![bet];
                }

                for i in 0..self.players.len() {
                    if i == client_id {
                        continue;
                    }

                    let player = match &self.players[i] {
                        Some(player) => player,
                        None => continue,
                    };

                    if player.state() == PlayerState::PlayingNextRound {
                        return vec![bet];
                    }
                }

                let (deal, shuffled) = self.deal();

                vec![bet, deal]
            }

            ClientMessage::Chat(_) | ClientMessage::Hello(_) => Vec::new(),
        }
    }
}
