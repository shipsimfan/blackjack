use crate::model::{Card, Rank, Shoe, Suit};
use std::num::NonZeroU8;

impl Shoe {
    /// Creates a new [`Shoe`]
    pub fn new(decks: NonZeroU8, rigged_cards: Vec<Card>) -> Self {
        let size = decks.get() as usize * 52;
        let mut cards = Vec::with_capacity(size);
        for _ in 0..decks.get() {
            for suit in Suit::ALL {
                for rank in Rank::ALL {
                    cards.push(Card::new(rank, suit));
                }
            }
        }

        Shoe {
            size,
            cards,
            cut_card: None,
            discard: Vec::with_capacity(size),
            rigged_cards,
        }
    }
}
