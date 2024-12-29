use crate::{messages::ServerMessage, model::BlackjackTable};

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

            ServerMessage::Error(_)
            | ServerMessage::GameState(_)
            | ServerMessage::Hello(_)
            | ServerMessage::Chat(_) => return false,
        }

        true
    }
}
