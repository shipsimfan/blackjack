use super::{ControlEvent, ControlsView};
use crate::TerminalEvent;

impl ControlsView {
    /// Handle an event from the terminal
    pub fn handle_terminal(&mut self, event: TerminalEvent) -> Option<ControlEvent> {
        match event {
            TerminalEvent::Character('\t') => Some(ControlEvent::SetChatActive),
            _ => None,
        }
    }
}
