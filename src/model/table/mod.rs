use crate::model::{Hand, Player, Shoe};
use std::num::{NonZeroU16, NonZeroU8};

mod game_state;
mod handle_message_result;

mod add_player;
mod change_state;
mod deal;
mod generate;
mod get;
mod handle_message;
mod new;
mod next_hand;
mod parse;
mod remove_player;
mod translate_message;

pub use game_state::GameState;
pub use handle_message_result::HandleMessageResult;

use super::Ratio;

/// A single table of blackjack
#[derive(Debug, Clone)]
pub struct BlackjackTable {
    /// The shoe the table is using
    shoe: Option<Shoe>,

    /// The available slots at the table and the players that fill them
    players: Box<[Option<Player>]>,

    /// The current state of the game
    state: GameState,

    /// The current hand of the dealer
    dealer_hand: Hand,

    /// The maximum bet a player can make
    max_bet: NonZeroU16,

    /// The minimum bet a player can make
    min_bet: NonZeroU16,

    /// The minimum required players to begin a round
    min_players: NonZeroU8,

    /// The minimum required human players to begin a round
    min_humans: u8,

    /// The maximum number of hands a single player may start a round with
    ///
    /// NOTE: The actual number of hands a player is playing may exceed this value due to splitting
    max_hands: NonZeroU8,

    /// The payout for blackjacks
    blackjack_payout: Ratio,

    /// Will the dealer hit on a soft 17?
    hit_soft_17: bool,
}
