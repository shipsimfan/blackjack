use super::{ChatEvent, ChatView};
use crate::{TerminalEvent, VirtualTerminal};

impl ChatView {
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        terminal: &mut VirtualTerminal,
    ) -> ChatEvent {
        match event {
            TerminalEvent::Character('\t') => return ChatEvent::SetControlsActive,
            _ => {}
        }

        match self.input.handle_event(&event, terminal) {
            Some(message) => {
                let message = message.to_owned();
                self.input.clear(terminal);
                ChatEvent::Message(message)
            }
            None => ChatEvent::None,
        }
    }
}
