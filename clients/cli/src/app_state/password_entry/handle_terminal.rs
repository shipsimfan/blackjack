use super::PasswordEntryState;
use crate::{TerminalEvent, VirtualTerminal};

impl PasswordEntryState {
    /// Handle a terminal event, returning if the program should exit
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        terminal: &mut VirtualTerminal,
    ) -> bool {
        if self.password_input.handle_event(&event, terminal).is_some() {
            terminal.write('\n');
            return true;
        }

        false
    }
}
