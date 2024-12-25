use crate::{GameState, TerminalEvent, VirtualTerminal};

impl GameState {
    /// Handle a terminal event, returning if the program should exit
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        virtual_terminal: &mut VirtualTerminal,
    ) -> bool {
        match self {
            GameState::Connecting(connecting) => {
                connecting.handle_terminal(event, virtual_terminal)
            }
        }
    }
}
