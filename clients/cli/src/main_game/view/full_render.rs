use super::View;
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Re-render the full terminal, even if nothing has changed
    pub fn full_render(&mut self, table: &BlackjackTable, terminal: &mut VirtualTerminal) {
        terminal.move_cursor_to(0, 0);

        // Write the server name
        let server_name_height = self.server_name.len().div_ceil(self.game_width);
        for y in 0..server_name_height {
            let start = y * self.game_width;
            let end = self.server_name.len().min(start + self.game_width);

            terminal.move_cursor_to(0, y);
            terminal.write(&self.server_name[start..end]);

            if end - start < self.game_width {
                for _ in 0..self.game_width - (end - start) {
                    terminal.write(' ');
                }
            }
        }

        // Write the bet amounts
        terminal.move_cursor_to(0, server_name_height + 1);
        let bet_amount = format!("  Bets: ${} - ${}", self.min_bet, self.max_bet);
        terminal.write(&bet_amount);
        for _ in 0..self.game_width - bet_amount.len() {
            terminal.write(' ');
        }

        // Clear the rest of the game screen
        for y in server_name_height + 2..terminal.height() {
            terminal.move_cursor_to(0, y);
            for _ in 0..self.game_width {
                terminal.write(' ');
            }
        }

        // Draw the vertical bar
        for y in 0..terminal.height() {
            terminal.move_cursor_to(self.game_width, y);
            terminal.write(" ｜ ");
        }

        self.chat.full_render(terminal);
    }
}
