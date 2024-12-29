use super::PlayerView;
use crate::VirtualTerminal;

impl PlayerView {
    /// Clear the area used below `y`
    pub fn clear(&mut self, y: usize, terminal: &mut VirtualTerminal) {
        let mut bottom_y = self.y + self.height();
        let target_y = y.max(self.y);

        while bottom_y >= target_y {
            terminal.move_cursor_to(0, bottom_y);
            terminal.write_blank(self.width);
            bottom_y -= 1;
        }
    }
}
