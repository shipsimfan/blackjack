mod all;
mod display;
mod from_u8;
mod unicode_base;

/// The suit of a given card
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Clubs = 0,
    Spades = 1,
    Diamonds = 2,
    Hearts = 3,
}
