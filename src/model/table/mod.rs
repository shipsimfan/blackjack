use crate::model::Player;
use std::num::{NonZeroU16, NonZeroU8};

mod game_state;

mod add_player;
mod generate;
mod get;
mod handle_message;
mod new;
mod parse;
mod remove_player;
mod translate_message;

pub use game_state::GameState;

/// A single table of blackjack
#[derive(Debug, Clone)]
pub struct BlackjackTable {
    /// The available slots at the table and the players that fill them
    players: Box<[Option<Player>]>,

    /// The current state of the game
    state: GameState,

    /// The maximum bet a player can make
    max_bet: NonZeroU16,

    /// The minimum bet a player can make
    min_bet: NonZeroU16,

    /// The minimum required players to begin a round
    min_players: NonZeroU8,

    /// The minimum required human players to begin a round
    min_humans: u8,
}
