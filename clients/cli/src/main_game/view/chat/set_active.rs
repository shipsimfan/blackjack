use super::ChatView;
use crate::VirtualTerminal;

impl ChatView {
    /// Change the active status of the chat window
    pub fn set_active(&mut self, active: bool, terminal: &mut VirtualTerminal) {
        if active {
            self.move_cursor(terminal);
            terminal.show_cursor();
        } else {
            terminal.hide_cursor();
        }
    }
}
