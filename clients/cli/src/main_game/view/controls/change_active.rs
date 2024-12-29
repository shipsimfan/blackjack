use super::ControlsView;

impl ControlsView {
    /// Set the chat active and re-render the appropriate controls
    pub fn set_chat_active(&mut self) {
        self.chat_active = true;
    }

    /// Set the controls active and re-render the appropriate controls
    pub fn set_controls_active(&mut self) {
        self.chat_active = false;
    }
}
