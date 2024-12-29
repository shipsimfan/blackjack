use super::ControlsView;

impl ControlsView {
    /// Is the chat currently active for input?
    pub fn chat_active(&self) -> bool {
        self.chat_active
    }
}
