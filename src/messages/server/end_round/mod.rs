mod generate;
mod new;
mod parse;

/// The round has ended
#[derive(Debug, Clone)]
pub struct EndRoundServerMessage {
    /// Should the dealer play?
    dealer_play: bool,
}
