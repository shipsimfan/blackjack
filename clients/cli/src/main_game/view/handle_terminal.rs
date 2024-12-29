use super::{chat::ChatEvent, controls::ControlEvent, View, ViewEvent};
use crate::{TerminalEvent, VirtualTerminal};

impl View {
    /// Handle an event from the terminal
    pub fn handle_terminal(
        &mut self,
        event: TerminalEvent,
        terminal: &mut VirtualTerminal,
    ) -> Option<ViewEvent> {
        let event = match self.controls.chat_active() {
            true => {
                return match self.chat.handle_terminal(event, terminal) {
                    ChatEvent::Message(message) => Some(ViewEvent::Chat(message)),
                    ChatEvent::SetControlsActive => {
                        self.chat.set_active(false, terminal);
                        self.controls.set_controls_active();
                        None
                    }
                    ChatEvent::None => None,
                }
            }
            false => match self.controls.handle_terminal(event) {
                Some(event) => event,
                None => return None,
            },
        };

        match event {
            ControlEvent::SetChatActive => {
                self.controls.set_chat_active();
                self.chat.set_active(true, terminal);
                None
            }
        }
    }
}
