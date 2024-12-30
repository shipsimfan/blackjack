use crate::{
    messages::{ClientMessage, PlayNextRoundServerMessage, ServerMessage},
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
                if player.state() != PlayerState::PlayingThisRound
                    && player.state() != play_next_round.as_state()
                {
                    vec![PlayNextRoundServerMessage::new(
                        client_id as _,
                        play_next_round.play_next_round,
                    )]
                } else {
                    Vec::new()
                }
            }

            ClientMessage::Chat(_) | ClientMessage::Hello(_) => Vec::new(),
        }
    }
}
