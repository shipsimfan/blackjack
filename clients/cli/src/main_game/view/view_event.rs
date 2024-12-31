use std::num::NonZeroU16;

/// An event that can happen from the view handling a terminal event
#[derive(Debug)]
pub enum ViewEvent {
    /// A chat message that should be sent
    Chat(String),

    /// The user wants to play next round
    PlayNextRound,

    /// The user does not want to play next round
    DontPlayNextRound,

    /// The user wants to place a bet
    PlaceBet(NonZeroU16),
}
