mod all;
mod display;
mod from_u8;
mod value;

/// The rank of a given card
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    Ace = 0x1,
    Two = 0x2,
    Three = 0x3,
    Four = 0x4,
    Five = 0x5,
    Six = 0x6,
    Seven = 0x7,
    Eight = 0x8,
    Nine = 0x9,
    Ten = 0xA,
    Jack = 0xB,
    Queen = 0xD,
    King = 0xE,
}
