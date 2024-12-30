use super::{ControlState, ControlsView};
use crate::TextInput;
use blackjack::model::GameState;

impl ControlsView {
    /// Creates a new [`ControlsView`]
    pub fn new(width: usize) -> Self {
        ControlsView {
            chat_active: false,
            state: ControlState::None,
            game_state: GameState::WaitingForPlayers,
            width,
            y: 0,
            selected_option: 0,
            last_selected_option: 0,
            bet_input: TextInput::new(
                5,
                "$",
                1,
                None,
                true,
                false,
                Some(width),
                Some(|c| c.is_ascii_digit()),
                None,
            ),
        }
    }
}
