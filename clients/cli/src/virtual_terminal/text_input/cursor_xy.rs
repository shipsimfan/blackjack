use crate::{TextInput, VirtualTerminal};

impl TextInput {
    /// Gets the absolute x-position of the cusor
    pub fn cursor_x_abs(&self) -> usize {
        self.cursor_x() + self.margin
    }

    /// Gets the absolute y-position of the cusor
    pub fn cursor_y_abs(&self, terminal: &VirtualTerminal) -> usize {
        self.cursor_y()
            + if self.at_bottom {
                terminal.height() - self.height
            } else {
                0
            }
    }

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
