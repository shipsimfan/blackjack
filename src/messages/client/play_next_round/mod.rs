mod as_state;
mod generate;
mod new;
mod parse;

/// Notice to the server to indicate if a player will be playing the next round
///
/// Does nothing if the player is actively playing a round
#[derive(Debug, Clone)]
pub struct PlayNextRoundClientMessage {
    /// Will the player be participating in the next round?
    pub play_next_round: bool,
}
