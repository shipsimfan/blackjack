use super::ControlsView;
use crate::VirtualTerminal;

impl ControlsView {
    /// Move the cursor to the correct spot
    pub fn move_cursor(&mut self, terminal: &mut VirtualTerminal) {
        terminal.move_cursor_to(
            self.bet_input.cursor_x_abs(),
            self.bet_input.cursor_y_abs(terminal),
        );
    }
}
