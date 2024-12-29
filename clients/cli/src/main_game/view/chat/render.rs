use super::ChatView;
use crate::virtual_terminal::VirtualTerminal;

impl ChatView {
    /// Render the messages to the terminal
    pub fn render(&mut self, terminal: &mut VirtualTerminal) {
        let base_y = self.height - self.rendered.len();
        for i in 0..self.rendered.len() {
            terminal.move_cursor_to(self.input.margin(), i + base_y);
            terminal.write(&self.rendered[i]);
        }
    }
}
