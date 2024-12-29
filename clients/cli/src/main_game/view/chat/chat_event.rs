/// An event that can occur while the chat handles a terminal event
#[derive(Debug)]
pub enum ChatEvent {
    /// The user entered a chat message
    Message(String),

    /// The user wants to switch to the game controls
    SetControlsActive,

    /// No event happened
    None,
}
