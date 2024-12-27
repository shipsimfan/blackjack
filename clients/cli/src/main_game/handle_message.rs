use crate::{Connection, MainGame, VirtualTerminal};
use blackjack::messages::ServerMessage;

impl MainGame {
    /// Handles a message from the server, returning true if the program should exit
    pub fn handle_message(
        &mut self,
        message: ServerMessage,
        terminal: &mut VirtualTerminal,
        connection: &mut Connection,
    ) -> bool {
        match message {
            _ => true,
        }
    }
}
