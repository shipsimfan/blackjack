use crate::{AppState, TerminalEvent, VirtualTerminal};

impl AppState {
    /// Handle a terminal event, returning if the program should exit
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        virtual_terminal: &mut VirtualTerminal,
    ) -> bool {
        match self {
            AppState::Connecting(connecting) => connecting.handle_terminal(event, virtual_terminal),
            AppState::PasswordEntry(password_entry) => {
                password_entry.handle_terminal(event, virtual_terminal)
            }
        }
    }
}
