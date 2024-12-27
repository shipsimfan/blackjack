use super::WaitForGameState;
use crate::virtual_terminal::VirtualTerminal;
use blackjack::{messages::ServerMessage, model::BlackjackTable};

impl WaitForGameState {
    /// Handle a connection message event, returning the initial game state and client id or
    /// [`None`] if an unexpected message was seen
    pub fn handle_message(
        &mut self,
        message: ServerMessage,
        terminal: &mut VirtualTerminal,
    ) -> Option<(u8, BlackjackTable)> {
        match message {
            ServerMessage::GameState(game_state) => {
                Some((game_state.client_id, game_state.unwrap()))
            }
            _ => {
                terminal.write("Error: unexpected message from server");
                None
            }
        }
    }
}
