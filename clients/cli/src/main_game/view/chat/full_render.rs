use super::ChatView;
use crate::VirtualTerminal;

impl ChatView {
    /// Re-render the full chat area, even if nothing has changed
    pub fn full_render(&mut self, terminal: &mut VirtualTerminal) {
        self.input.render(terminal);

        todo!("Render messages from bottom up");

        todo!("If remaining space, clear it");
    }
}
