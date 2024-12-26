use super::password_entry::PasswordEntryState;
use crate::{virtual_terminal::VirtualTerminal, AppState};
use blackjack::messages::ServerMessage;

impl AppState {
    /// Handle a connection message event, returning [`None`] if the program should exit
    pub fn handle_message(
        self,
        message: ServerMessage,
        virtual_terminal: &mut VirtualTerminal,
    ) -> Option<AppState> {
        Some(match self {
            AppState::Connecting(mut connecting) => {
                match connecting.handle_message(message, virtual_terminal) {
                    Some(true) => AppState::PasswordEntry(PasswordEntryState::new(
                        connecting.unwrap(),
                        virtual_terminal,
                    )),
                    Some(false) => todo!("Switch to waiting game state"),
                    None => AppState::Connecting(connecting),
                }
            }

            AppState::PasswordEntry(mut password_entry) => {
                if password_entry.handle_message(message, virtual_terminal) {
                    return None;
                }

                AppState::PasswordEntry(password_entry)
            }
        })
    }
}
