use super::{ChatView, MAX_CHAT_MESSAGES};
use crate::VirtualTerminal;

impl ChatView {
    /// Add a message to the chat history and re-render
    pub fn add_message(&mut self, message: String, terminal: &mut VirtualTerminal) {
        assert!(message.len() > 0);

        // Compute new message
        let height = message.len().div_ceil(self.width);
        for i in 0..height {
            if self.rendered.len() == self.height {
                self.rendered.pop_front();
            }

            let start = i * self.width;
            let end = message.len().min(start + self.width);
            self.rendered.push_back(message[start..end].to_owned());
        }

        // Update history
        if self.history.len() == MAX_CHAT_MESSAGES {
            self.history.pop_front();
        }
        self.history.push_back(message);

        // Render
        self.render(terminal);
    }
}
