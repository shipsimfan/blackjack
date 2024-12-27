use super::View;
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Re-render the full terminal, even if nothing has changed
    pub fn full_render(&mut self, table: &BlackjackTable, terminal: &mut VirtualTerminal) {
        todo!("Render server name");
        todo!("Render betting amounts");

        todo!("Clear the game half");

        todo!("Render vertical bar");

        self.chat.full_render(terminal);
    }
}
