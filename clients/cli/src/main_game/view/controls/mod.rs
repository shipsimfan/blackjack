mod control_event;

mod change_active;
mod get;
mod handle_terminal;
mod new;

pub use control_event::ControlEvent;

/// The view of the current controls
pub struct ControlsView {
    /// Is the chat currently active?
    chat_active: bool,
}
