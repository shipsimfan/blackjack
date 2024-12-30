use crate::{
    messages::ServerMessage,
    model::{BlackjackTable, PlayerState},
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
                if player.state() != PlayerState::PlayingThisRound {
                    player.set_state(play_next_round.as_state());
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
