use super::ChatView;
use crate::VirtualTerminal;

impl ChatView {
    /// Re-render the full chat area, even if nothing has changed
    pub fn full_render(&mut self, active: bool, terminal: &mut VirtualTerminal) {
        self.input.render(active, terminal);

        // Clear the message area
        for y in 0..self.height {
            terminal.move_cursor_to(self.input.margin(), y);
            terminal.write_blank(self.width);
        }

        // Draw chat box sepearator
        terminal.move_cursor_to(self.input.margin() - 2, self.height);
        terminal.write('├');
        for _ in 0..self.width + 1 {
            terminal.write('─');
        }

        // Re-compute all messages into `rendered`
        self.rendered.truncate(0);
        self.rendered.reserve(self.height);
        'message_loop: for message in self.history.iter().rev() {
            let height = message.len().div_ceil(self.width);
            for i in (0..height).rev() {
                if self.rendered.len() == self.height {
                    break 'message_loop;
                }

                let start = i * self.width;
                let end = message.len().min(start + self.width);
                let mut string = message[start..end].to_owned();
                if string.len() < self.width {
                    string.extend(std::iter::repeat_n(' ', self.width - string.len()));
                }
                self.rendered.push_front(string);
            }
        }

        self.render(terminal);
    }
}
