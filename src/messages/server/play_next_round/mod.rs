mod as_state;
mod generate;
mod new;
mod parse;

/// A player has changed if they will be playing in the next round
#[derive(Debug, Clone)]
pub struct PlayNextRoundServerMessage {
    /// The client whose state has changed
    pub client: u8,

    /// Will the player be participating in the next round?
    pub play_next_round: bool,
}
