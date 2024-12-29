/// An event that the happen with the controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlEvent {
    /// The user wants to switch to chat input
    SetChatActive,
}
