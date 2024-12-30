mod generate;
mod parse;

/// The current state of a player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    /// The player is currently sitting out
    NotPlaying,

    /// The player will be joining the next round
    PlayingNextRound,

    /// The player is actively in the current round
    PlayingThisRound,
}
