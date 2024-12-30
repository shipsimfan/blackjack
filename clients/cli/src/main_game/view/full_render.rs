use super::View;
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Re-render the full terminal, even if nothing has changed
    pub fn full_render(
        &mut self,
        table: &BlackjackTable,
        local_id: usize,
        terminal: &mut VirtualTerminal,
    ) {
        terminal.move_cursor_to(0, 0);

        // Write the server name
        for y in 0..self.server_name_height {
            let start = y * self.game_width;
            let end = self.server_name.len().min(start + self.game_width);

            terminal.move_cursor_to(0, y);
            terminal.write(&self.server_name[start..end]);

            if end - start < self.game_width {
                terminal.write_blank(self.game_width - (end - start));
            }
        }

        // Write the bet amounts
        terminal.move_cursor_to(0, self.server_name_height + 1);
        let bet_amount = format!("  Bets: ${} - ${}", self.min_bet, self.max_bet);
        terminal.write(&bet_amount);
        terminal.write_blank(self.game_width - bet_amount.len());

        // Write blank line
        terminal.move_cursor_to(0, self.server_name_height + 2);
        terminal.write_blank(self.game_width);

        // Write the "Dealer" header
        terminal.move_cursor_to(0, self.server_name_height + 3);
        let dealer_header = "Dealer";
        terminal.write(&dealer_header);
        terminal.write_blank(self.game_width - dealer_header.len());

        // Clear the rest of the game screen
        for y in self.dealer_hand_y..terminal.height() - 1 {
            terminal.move_cursor_to(0, y);
            terminal.write_blank(self.game_width);
        }

        // Draw the vertical bar
        for y in 0..terminal.height() {
            terminal.move_cursor_to(self.game_width, y);
            terminal.write(" │ ");
        }

        self.chat.full_render(self.controls.chat_active(), terminal);
        self.players.truncate();
        self.controls.render(true, table, local_id, terminal);

        self.render(table, local_id, terminal);
    }
}
