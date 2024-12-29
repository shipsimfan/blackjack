use super::ChatView;
use crate::VirtualTerminal;

impl ChatView {
    /// Move the cursor to the correct spot
    pub fn move_cursor(&mut self, terminal: &mut VirtualTerminal) {
        /*terminal.use_main_buffer();
        panic!(
            "{}:{}",
            self.input.cursor_x_abs(),
            self.input.cursor_y_abs(terminal)
        );*/

        terminal.move_cursor_to(self.input.cursor_x_abs(), self.input.cursor_y_abs(terminal));
    }
}