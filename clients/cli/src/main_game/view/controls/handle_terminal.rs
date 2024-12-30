use super::{ControlEvent, ControlsView};
use crate::{SpecialKey, TerminalEvent, VirtualTerminal};
use blackjack::model::BlackjackTable;

impl ControlsView {
    /// Handle an event from the terminal
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        table: &BlackjackTable,
        local_id: usize,
        terminal: &mut VirtualTerminal,
    ) -> Option<ControlEvent> {
        match event {
            TerminalEvent::Character('\t') => Some(ControlEvent::SetChatActive),
            TerminalEvent::Character('\r') => self.state.to_control_event(self.selected_option),
            TerminalEvent::SpecialKey(SpecialKey::LeftArrow) => {
                if self.selected_option > 0 {
                    self.selected_option -= 1;
                    self.render(false, table, local_id, terminal);
                }
                None
            }
            TerminalEvent::SpecialKey(SpecialKey::RightArrow) => {
                if self.state.options().len() != 0
                    && self.selected_option < self.state.options().len() - 1
                {
                    self.selected_option += 1;
                    self.render(false, table, local_id, terminal);
                }
                None
            }
            _ => None,
        }
    }
}
