use super::View;
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Render the parts of the view that have changed
    pub fn render(&mut self, table: &BlackjackTable, terminal: &mut VirtualTerminal) {
        terminal.hide_cursor();

        self.players.render(table.players(), terminal);

        if self.controls.chat_active() {
            self.chat.move_cursor(terminal);
            terminal.show_cursor();
        }
    }
}
