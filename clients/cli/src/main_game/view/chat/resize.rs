use super::ChatView;
use crate::VirtualTerminal;

impl ChatView {
    /// Resize the chat view to `terminal`
    pub fn resize(&mut self, terminal: &mut VirtualTerminal) {
        self.input.resize(false, terminal);
        self.height = terminal.height() - self.input.height();
        self.width = terminal.width() - self.input.margin();
    }
}
