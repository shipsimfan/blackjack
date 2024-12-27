use super::View;
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Resizes the view to match `terminal`
    pub fn resize(&mut self, table: &BlackjackTable, terminal: &mut VirtualTerminal) {
        self.chat.resize(terminal);

        self.full_render(table, terminal);
    }
}
