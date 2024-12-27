use super::ChatView;
use crate::VirtualTerminal;

impl ChatView {
    /// Resize the chat view to `terminal`
    pub fn resize(&mut self, terminal: &mut VirtualTerminal) {
        self.input.resize(terminal);
        self.height = terminal.height() - self.input.height();
        self.width = terminal.width() - self.input.margin();
        self.rendered.truncate(0);
        self.rendered.reserve(self.height);
    }
}
