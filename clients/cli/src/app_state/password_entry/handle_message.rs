use super::PasswordEntryState;
use crate::virtual_terminal::VirtualTerminal;
use blackjack::messages::ServerMessage;

impl PasswordEntryState {
    /// Handle a connection message event, returning true if the program should exit
    pub fn handle_message(
        &mut self,
        _: ServerMessage,
        virtual_terminal: &mut VirtualTerminal,
    ) -> bool {
        virtual_terminal.write("Error: unexpected message");
        true
    }
}
