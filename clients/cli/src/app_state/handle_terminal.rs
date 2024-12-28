use super::WaitForGameState;
use crate::{AppState, Connection, TerminalEvent, VirtualTerminal};

impl AppState {
    /// Handle a terminal event, returning if the program should exit
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        terminal: &mut VirtualTerminal,
        connection: &mut Connection,
    ) -> bool {
        match self {
            AppState::PasswordEntry(password_entry) => {
                if password_entry.handle_terminal(event, terminal) {
                    let mut server_name = String::new();
                    std::mem::swap(&mut server_name, &mut password_entry.server_name);
                    *self = AppState::WaitForGameState(WaitForGameState::new(
                        connection,
                        password_entry.username(),
                        Some(password_entry.password()),
                        server_name,
                    ));
                }

                false
            }
            AppState::MainGame(main_game) => main_game.handle_terminal(event, terminal, connection),
            AppState::WaitForGameState(_) | AppState::Connecting(_) => false,
        }
    }
}
