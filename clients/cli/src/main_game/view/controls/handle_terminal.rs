use super::{ControlEvent, ControlState, ControlsView};
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
        if event == TerminalEvent::Character('\t') {
            return Some(ControlEvent::SetChatActive);
        }

        if self.state == ControlState::Betting {
            if event == TerminalEvent::Character('\x1B') {
                return Some(ControlEvent::DontPlayNextRound);
            }

            if let Some(bet) = self.bet_input.handle_event(&event, terminal) {
                if bet.len() == 0 {
                    return None;
                }

                let bet: usize = bet.parse().unwrap();
                if bet < table.min_bet() as _ || bet > table.max_bet() as _ {
                    return None;
                }

                return self.state.to_control_event(bet);
            }

            return None;
        }

        match event {
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
