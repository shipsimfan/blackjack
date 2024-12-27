use super::WaitForGameState;
use crate::virtual_terminal::VirtualTerminal;
use blackjack::{messages::ServerMessage, model::BlackjackTable};

impl WaitForGameState {
    /// Handle a connection message event, returning the initial game state or [`None`] if an
    /// unexpected message was seen
    pub fn handle_message(
        &mut self,
        _: ServerMessage,
        _: &mut VirtualTerminal,
    ) -> Option<BlackjackTable> {
        todo!()
    }
}
