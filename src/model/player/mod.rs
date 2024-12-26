use crate::messages::Username;

mod generate;
mod new;
mod parse;

/// A single player at a table of blackjack
#[derive(Debug, Clone)]
pub struct Player {
    /// The name of the player
    username: Username<'static>,

    /// Is this player an AI?
    ai: bool,
}
