use crate::{TextInput, VirtualTerminal};

impl TextInput {
    /// Renders this text input to `terminal`
    pub fn render(&mut self, terminal: &mut VirtualTerminal) {
        terminal.hide_cursor();

        // Move cursor to start
        if self.at_bottom {
            terminal.move_cursor_to(self.margin, terminal.height() - self.height);
        } else {
            terminal.move_cursor_to_x(self.margin);
            terminal.move_cursor_y(-(self.last_cursor_y() as isize));
        }

        // Write prompt
        terminal.write(self.prompt);

        // Write value
        let (mut text_x, mut text_y) = if self.value.len() <= self.first_line_width {
            self.write_text(&self.value, terminal);
            (self.value.len() + self.prompt.len(), 0)
        } else {
            self.write_text(&self.value[..self.first_line_width], terminal);

            let height = (self.value.len() - self.first_line_width).div_ceil(self.width);
            for y in 0..height {
                let start = self.first_line_width + y * self.width;
                let end = self
                    .value
                    .len()
                    .min(self.first_line_width + (y + 1) * self.width);

                terminal.write('\n');
                terminal.move_cursor_to_x(self.margin);
                self.write_text(&self.value[start..end], terminal);
            }

            (
                (self.value.len() - self.first_line_width) % self.width,
                height,
            )
        };

        // Write blank
        while text_y < self.height {
            for _ in 0..self.width - text_x {
                terminal.write(' ');
            }

            text_y += 1;
            text_x = 0;

            if text_y != self.height {
                terminal.write('\n');
            }
            terminal.move_cursor_to_x(self.margin);
        }

        // Move cursor to proper spot
        terminal.move_cursor_to_x(self.margin);
        terminal.move_cursor(
            self.cursor_x() as isize,
            -((self.height - self.cursor_y() - 1) as isize),
        );

        self.last_cursor = self.cursor;

        terminal.show_cursor();
    }

    /// Writes `text` to `terminal`, hiding it if nescessary
    fn write_text(&self, text: &[u8], terminal: &mut VirtualTerminal) {
        match self.hide_character {
            Some(c) => {
                for _ in 0..text.len() {
                    terminal.write(c as char);
                }
            }
            None => terminal.write(unsafe { std::str::from_utf8_unchecked(text) }),
        }
    }
}
