use crate::{messages::Username, model::Hand};

mod state;

mod generate;
mod get;
mod hands;
mod new;
mod parse;
mod payout;
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

    /// The set of hands this player currently has
    hands: Vec<Hand>,

    /// The total amount of money this player has made
    total_earnings: i64,

    /// The amount of money this player won or lost last round, if they played
    last_round_earnings: Option<i32>,
}
