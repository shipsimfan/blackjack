use super::ChatView;
use crate::VirtualTerminal;

impl ChatView {
    /// Re-render the full chat area, even if nothing has changed
    pub fn full_render(&mut self, terminal: &mut VirtualTerminal) {
        self.input.render(false, terminal);

        // Clear the message area
        for y in 0..self.height {
            terminal.move_cursor_to(self.input.margin(), y);
            for _ in 0..self.width {
                terminal.write(' ');
            }
        }
    }
}
