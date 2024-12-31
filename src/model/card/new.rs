use crate::model::{Card, Rank, Suit};

impl Card {
    /// Creates a new [`Card`]
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card { rank, suit }
    }
}
