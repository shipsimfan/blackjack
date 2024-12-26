use crate::model::Player;

mod add_player;
mod generate;
mod get;
mod new;
mod parse;
mod remove_player;

/// A single table of blackjack
#[derive(Debug, Clone)]
pub struct BlackjackTable {
    /// The available slots at the table and the players that fill them
    players: Box<[Option<Player>]>,
}
