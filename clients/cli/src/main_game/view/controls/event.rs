use std::num::NonZeroU16;

/// An event that the happen with the controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlEvent {
    /// The user wants to switch to chat input
    SetChatActive,

    /// The user wants to play next round
    PlayNextRound,

    /// The user does not want to play next round
    DontPlayNextRound,

    /// The user wants to place a bet
    PlaceBet(NonZeroU16),
}
