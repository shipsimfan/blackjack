use super::View;
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Resizes the view to match `terminal`
    pub fn resize(
        &mut self,
        table: &BlackjackTable,
        local_id: usize,
        terminal: &mut VirtualTerminal,
    ) {
        self.chat.resize(self.controls.chat_active(), terminal);
        self.controls.resize(terminal);

        self.full_render(table, local_id, terminal);
    }
}
