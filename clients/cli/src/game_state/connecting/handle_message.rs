use super::Connecting;
use crate::virtual_terminal::VirtualTerminal;
use blackjack::messages::ServerMessage;

impl Connecting {
    /// Handle a connection message event
    pub fn handle_message(
        &mut self,
        message: ServerMessage,
        virtual_terminal: &mut VirtualTerminal,
    ) {
        virtual_terminal.write(format_args!("Received message {}\n", message.tag()));
    }
}
