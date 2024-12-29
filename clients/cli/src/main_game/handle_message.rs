use crate::{MainGame, VirtualTerminal};
use blackjack::messages::ServerMessage;

impl MainGame {
    /// Handles a message from the server, returning true if the program should exit
    pub fn handle_message(
        &mut self,
        message: ServerMessage,
        terminal: &mut VirtualTerminal,
    ) -> bool {
        match &message {
            ServerMessage::ClientConnected(connecting) => {
                self.view.add_message(
                    format!("{} connected", connecting.player.username()),
                    terminal,
                );
            }
            ServerMessage::ClientDisconnected(disconnecting) => {
                self.view.add_message(
                    format!(
                        "{} disconnected",
                        self.table.player(disconnecting.id as _).username()
                    ),
                    terminal,
                );
            }
            ServerMessage::Chat(chat) => {
                self.view.add_message(
                    format!(
                        "[{}] {}",
                        self.table.player(chat.client as _).username(),
                        chat.message
                    ),
                    terminal,
                );
            }
            ServerMessage::Error(_) | ServerMessage::GameState(_) | ServerMessage::Hello(_) => {
                return true
            }
        }

        if self.table.handle_message(message) {
            self.view.render(&self.table, terminal);
        }

        false
    }
}
