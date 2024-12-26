use crate::TextInput;

impl TextInput {
    /// Gets the cursor's relative x position
    pub fn cursor_x(&self) -> usize {
        if self.cursor < self.first_line_width {
            return self.cursor + self.prompt.len();
        }

        (self.cursor - self.first_line_width) % self.width
    }

    /// Gets the cursor's relative y position
    pub fn cursor_y(&self) -> usize {
        if self.cursor < self.first_line_width {
            return 0;
        }

        (self.cursor - self.first_line_width) / self.width + 1
    }

    /// Gets the cursor's relative y position the last time a render occurred
    pub(super) fn last_cursor_y(&self) -> usize {
        if self.last_cursor < self.first_line_width {
            return 0;
        }

        (self.last_cursor - self.first_line_width) / self.width + 1
    }
}
