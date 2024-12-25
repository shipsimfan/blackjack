use super::Connecting;
use crate::{TerminalEvent, VirtualTerminal};

impl Connecting {
    /// Handle a terminal event, returning if the program should exit
    pub fn handle_terminal(&mut self, _: TerminalEvent, _: &mut VirtualTerminal) -> bool {
        false
    }
}
