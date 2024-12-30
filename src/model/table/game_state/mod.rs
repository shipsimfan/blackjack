mod generate;
mod get;
mod parse;

/// The current state of the table
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Waiting for players to join the next round
    WaitingForPlayers,

    /// Waiting for players to place bets before beggining the next round
    WaitingForBets,

    /// Waiting for a player to make a decision
    WaitingForPlayer(u8),
}
