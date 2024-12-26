use super::Connecting;
use crate::virtual_terminal::VirtualTerminal;
use blackjack::messages::ServerMessage;

impl Connecting {
    /// Handle a connection message event, returning [`None`] if the program should exit, or a
    /// boolean indicating if a password is required by the server.
    pub fn handle_message(
        &mut self,
        message: ServerMessage,
        terminal: &mut VirtualTerminal,
    ) -> Option<bool> {
        match message {
            ServerMessage::Hello(hello) => {
                terminal.write(format_args!(
                    "Connected to {} ({} v{})\n",
                    hello.server_name(),
                    hello.server_application_name(),
                    hello.server_version()
                ));

                Some(hello.password_required())
            }
            _ => {
                terminal.write("Error: unexpected message");
                None
            }
        }
    }
}
