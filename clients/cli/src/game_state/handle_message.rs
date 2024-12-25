use crate::{virtual_terminal::VirtualTerminal, GameState};
use blackjack::messages::ServerMessage;

impl GameState {
    /// Handle a connection message event, returning if the program should exit
    pub fn handle_message(
        &mut self,
        message: ServerMessage,
        virtual_terminal: &mut VirtualTerminal,
    ) -> bool {
        match self {
            GameState::Connecting(connecting) => {
                match connecting.handle_message(message, virtual_terminal) {
                    Some(true) => todo!("Switch to password input state"),
                    Some(false) => todo!("Switch to waiting game state"),
                    None => false,
                }
            }
        }
    }
}
