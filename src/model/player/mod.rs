use crate::messages::Username;

mod state;

mod generate;
mod get;
mod new;
mod parse;
mod set;

pub use state::PlayerState;

/// A single player at a table of blackjack
#[derive(Debug, Clone)]
pub struct Player {
    /// The name of the player
    username: Username<'static>,

    /// Is this player an AI?
    ai: bool,

    /// The current state of this player
    state: PlayerState,
}
