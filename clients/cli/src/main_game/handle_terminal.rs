use crate::{Connection, MainGame, TerminalEvent, VirtualTerminal};

impl MainGame {
    /// Handle a terminal event, returning true if the program should exit
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        terminal: &mut VirtualTerminal,
        connection: &mut Connection,
    ) -> bool {
        false
    }
}
