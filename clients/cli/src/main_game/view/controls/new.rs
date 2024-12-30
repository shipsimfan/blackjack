use super::{ControlState, ControlsView};

impl ControlsView {
    /// Creates a new [`ControlsView`]
    pub fn new(width: usize) -> Self {
        ControlsView {
            chat_active: false,
            state: ControlState::None,
            width,
            y: 0,
            selected_option: 0,
            last_selected_option: 0,
        }
    }
}
