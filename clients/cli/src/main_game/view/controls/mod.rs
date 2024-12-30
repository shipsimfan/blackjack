use blackjack::model::GameState;

mod event;

mod change_active;
mod get;
mod handle_terminal;
mod new;
mod render;
mod resize;
mod state;

pub use event::ControlEvent;
pub use state::ControlState;

/// The view of the current controls
pub struct ControlsView {
    /// Is the chat currently active?
    chat_active: bool,

    /// The current state of the controls
    state: ControlState,

    /// The currently displayed game state
    game_state: GameState,

    /// The y-level to render at
    y: usize,

    /// The width of the area to print in
    width: usize,

    /// The currently selected option
    selected_option: usize,

    /// The last selected option
    last_selected_option: usize,
}
