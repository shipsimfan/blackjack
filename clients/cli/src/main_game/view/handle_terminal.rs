use super::{chat::ChatEvent, controls::ControlEvent, View, ViewEvent};
use crate::{TerminalEvent, VirtualTerminal};
use blackjack::model::BlackjackTable;

impl View {
    /// Handle an event from the terminal
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        table: &BlackjackTable,
        local_id: usize,
        terminal: &mut VirtualTerminal,
    ) -> Option<ViewEvent> {
        let event = match self.controls.chat_active() {
            true => {
                return match self.chat.handle_terminal(event, terminal) {
                    ChatEvent::Message(message) => Some(ViewEvent::Chat(message)),
                    ChatEvent::SetControlsActive => {
                        self.chat.set_active(false, terminal);
                        self.controls.set_controls_active();
                        self.controls.render(true, table, local_id, terminal);
                        None
                    }
                    ChatEvent::None => None,
                };
            }
            false => match self
                .controls
                .handle_terminal(event, table, local_id, terminal)
            {
                Some(event) => event,
                None => return None,
            },
        };

        match event {
            ControlEvent::SetChatActive => {
                self.controls.set_chat_active();
                self.controls.render(true, table, local_id, terminal);
                self.chat.set_active(true, terminal);
                None
            }
            ControlEvent::PlayNextRound => Some(ViewEvent::PlayNextRound),
            ControlEvent::DontPlayNextRound => Some(ViewEvent::DontPlayNextRound),
            ControlEvent::PlaceBet(bet) => Some(ViewEvent::PlaceBet(bet)),
            ControlEvent::Hit => Some(ViewEvent::Hit),
            ControlEvent::Stand => Some(ViewEvent::Stand),
        }
    }
}
