use super::{controls::ControlState, View};
use crate::VirtualTerminal;
use blackjack::model::BlackjackTable;

impl View {
    /// Render the parts of the view that have changed
    pub fn render(
        &mut self,
        table: &BlackjackTable,
        local_id: usize,
        force: bool,
        terminal: &mut VirtualTerminal,
    ) {
        terminal.hide_cursor();

        self.dealer
            .render(table.dealer(), self.dealer_hand_y, force, terminal);
        self.players
            .render(table.players(), table.max_bet(), force, terminal);
        self.controls.render(force, table, local_id, terminal);

        if self.controls.chat_active() {
            self.chat.move_cursor(terminal);
            terminal.show_cursor();
        } else if self.controls.state() == ControlState::Betting {
            self.controls.move_cursor(terminal);
            terminal.show_cursor();
        }
    }
}
