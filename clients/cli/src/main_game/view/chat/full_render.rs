use super::ChatView;
use crate::VirtualTerminal;

impl ChatView {
    /// Re-render the full chat area, even if nothing has changed
    pub fn full_render(&mut self, terminal: &mut VirtualTerminal) {
        self.input.render(false, terminal);

        // Clear the message area
        for y in 0..self.height - 1 {
            terminal.move_cursor_to(self.input.margin(), y);
            terminal.write_blank(self.width);
        }

        // Draw chat box sepearator
        terminal.move_cursor_to(self.input.margin() - 2, self.height - 1);
        terminal.write('├');
        for _ in 0..self.width + 1 {
            terminal.write('─');
        }
    }
}
