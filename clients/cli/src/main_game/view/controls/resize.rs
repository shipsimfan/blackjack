use super::ControlsView;
use crate::VirtualTerminal;

impl ControlsView {
    pub fn resize(&mut self, terminal: &mut VirtualTerminal) {
        self.y = terminal.height() - 2;
        self.bet_input.resize(false, terminal);
    }
}
