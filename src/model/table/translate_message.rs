use crate::{
    messages::{
        ClientMessage, HitServerMessage, PlaceBetServerMessage, PlayNextRoundServerMessage,
        ServerMessage, ShuffleServerMessage, StandServerMessage,
    },
    model::{BlackjackTable, PlayerState},
};

impl BlackjackTable {
    /// Translates a [`ClientMessage`] into zero or more corresponding [`ServerMessage`]s
    pub fn translate_message<'a>(
        &mut self,
        client_id: usize,
        message: &'a ClientMessage,
    ) -> Option<(ServerMessage<'a>, Option<ServerMessage<'a>>)> {
        match message {
            ClientMessage::PlayNextRound(play_next_round) => {
                let player = self.player(client_id);
                if player.state() == PlayerState::PlayingThisRound
                    || player.state() == play_next_round.as_state()
                {
                    return None;
                }

                Some((
                    PlayNextRoundServerMessage::new(
                        client_id as _,
                        play_next_round.play_next_round,
                    ),
                    None,
                ))
            }
            ClientMessage::PlaceBet(place_bet) => {
                let player = self.player(client_id);
                if player.state() != PlayerState::PlayingNextRound
                    || self.state.is_round_active()
                    || place_bet.bet < self.min_bet
                    || place_bet.bet > self.max_bet
                {
                    return None;
                }

                Some((
                    PlaceBetServerMessage::new(client_id as _, place_bet.bet),
                    None,
                ))
            }
            ClientMessage::Hit(_) => {
                let player = self.current_player()?;
                if client_id != player {
                    return None;
                }

                self.shoe.as_mut().map(|shoe| {
                    let (card, shuffle) = shoe.draw();
                    (
                        HitServerMessage::new(card),
                        if shuffle {
                            Some(ShuffleServerMessage::new())
                        } else {
                            None
                        },
                    )
                })
            }
            ClientMessage::Stand(_) => {
                let player = self.current_player()?;
                if client_id != player {
                    return None;
                }

                Some((StandServerMessage::new(), None))
            }

            ClientMessage::Chat(_) | ClientMessage::Hello(_) => None,
        }
    }
}
