use super::View;
use crate::VirtualTerminal;

impl View {
    /// Add a message to the chat
    pub fn add_message(&mut self, message: String, terminal: &mut VirtualTerminal) {
        self.chat.add_message(message, terminal);

        if self.controls.chat_active() {
            self.chat.move_cursor(terminal);
        }
    }
}
