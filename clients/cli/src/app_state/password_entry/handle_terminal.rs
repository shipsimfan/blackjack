use super::PasswordEntryState;
use crate::{TerminalEvent, VirtualTerminal};

impl PasswordEntryState {
    /// Handle a terminal event, returning if the program should exit
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        terminal: &mut VirtualTerminal,
    ) -> bool {
        if let Some(input) = self.password_input.handle_event(&event, terminal) {
            if input.len() == 0 {
                return false;
            }

            terminal.write('\n');
            return true;
        }

        false
    }
}
