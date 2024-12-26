use crate::model::BlackjackTable;

mod generate;
mod parse;
mod to_static;
mod unwrap;

/// The current state of a blackjack game
#[derive(Debug, Clone)]
pub enum GameStateServerMessage<'a> {
    /// The state is owned
    Owned(BlackjackTable),

    /// The state is borrowed
    Borrowed(&'a BlackjackTable),
}
