use crate::model::Player;
use std::num::NonZeroU16;

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

    /// The maximum bet a player can make
    max_bet: NonZeroU16,

    /// The minimum bet a player can make
    min_bet: NonZeroU16,
}
