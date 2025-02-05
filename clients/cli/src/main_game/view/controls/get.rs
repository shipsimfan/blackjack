use super::{ControlState, ControlsView};

impl ControlsView {
    /// Is the chat currently active for input?
    pub fn chat_active(&self) -> bool {
        self.chat_active
    }

    /// Gets the current state of the controls
    pub fn state(&self) -> ControlState {
        self.state
    }
}
