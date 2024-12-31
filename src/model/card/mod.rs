mod rank;
mod suit;

mod back;
mod display;
mod generate;
mod new;
mod parse;

pub use rank::Rank;
pub use suit::Suit;

/// A single playing card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    /// The rank of this card
    pub rank: Rank,

    /// The suit of this card
    pub suit: Suit,
}
