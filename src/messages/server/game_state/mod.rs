use crate::{messages::RefCow, model::BlackjackTable};

mod generate;
mod new;
mod parse;
mod to_static;
mod unwrap;

/// The current state of a blackjack game
#[derive(Debug, Clone)]
pub struct GameStateServerMessage<'a> {
    /// The id of the connecting client
    pub client_id: u8,

    /// The state of the game
    pub table: RefCow<'a, BlackjackTable>,
}
