use super::{PasswordEntryState, WaitForGameState};
use crate::{AppState, Connection, MainGame, VirtualTerminal};
use blackjack::messages::ServerMessage;

impl AppState {
    /// Handle a connection message event, returning [`None`] if the program should exit
    pub fn handle_message(
        self,
        message: ServerMessage,
        terminal: &mut VirtualTerminal,
        connection: &mut Connection,
    ) -> Option<AppState> {
        Some(match self {
            AppState::Connecting(mut connecting) => {
                match connecting.handle_message(message, terminal) {
                    Some((true, server_name)) => AppState::PasswordEntry(PasswordEntryState::new(
                        connecting.unwrap(),
                        server_name,
                        terminal,
                    )),
                    Some((false, server_name)) => AppState::WaitForGameState(
                        WaitForGameState::new(connection, connecting.unwrap(), None, server_name),
                    ),
                    None => AppState::Connecting(connecting),
                }
            }

            AppState::PasswordEntry(mut password_entry) => {
                if password_entry.handle_message(message, terminal) {
                    return None;
                }

                AppState::PasswordEntry(password_entry)
            }
            AppState::WaitForGameState(mut wait_for_game_state) => {
                match wait_for_game_state.handle_message(message, terminal) {
                    Some((client_id, model)) => AppState::MainGame(MainGame::new(
                        client_id,
                        model,
                        wait_for_game_state.server_name,
                        terminal,
                    )),
                    None => return None,
                }
            }
            AppState::MainGame(mut main_game) => {
                if main_game.handle_message(message, terminal) {
                    return None;
                }

                AppState::MainGame(main_game)
            }
        })
    }
}
