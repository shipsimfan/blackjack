use crate::{virtual_terminal::VirtualTerminal, GameState};
use blackjack::messages::ServerMessage;

impl GameState {
    /// Handle a connection message event
    pub fn handle_message(
        &mut self,
        message: ServerMessage,
        virtual_terminal: &mut VirtualTerminal,
    ) {
        match self {
            GameState::Connecting(connecting) => {
                connecting.handle_message(message, virtual_terminal)
            }
        }
    }
}
