use crate::{
    messages::{ClientMessage, PlaceBetServerMessage, PlayNextRoundServerMessage, ServerMessage},
    model::{BlackjackTable, PlayerState},
};

impl BlackjackTable {
    /// Translates a [`ClientMessage`] into zero or more corresponding [`ServerMessage`]s
    pub fn translate_message<'a>(
        &self,
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

                vec![PlayNextRoundServerMessage::new(
                    client_id as _,
                    play_next_round.play_next_round,
                )]
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

                // TODO: Check if all players would have placed bets at this point. If so, deal a
                // new hand

                vec![PlaceBetServerMessage::new(client_id as _, place_bet.bet)]
            }

            ClientMessage::Chat(_) | ClientMessage::Hello(_) => Vec::new(),
        }
    }
}
