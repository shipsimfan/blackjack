use crate::{TextInput, VirtualTerminal};

impl TextInput {
    /// Re-calculates the required input size and re-renders
    pub fn resize(&mut self, terminal: &mut VirtualTerminal) {
        self.width = terminal.width() - self.margin;
        self.first_line_width = self.width - self.prompt.len();

        if self.first_line_width > self.max_length {
            self.height = 1;
            return;
        }

        self.height = (self.max_length - self.first_line_width).div_ceil(self.width) + 1;

        self.render(terminal);
    }
}
