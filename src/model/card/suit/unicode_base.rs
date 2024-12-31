use crate::model::Suit;

impl Suit {
    /// Gets the base character in unicode for this [`Suit`]
    pub fn unicode_base(&self) -> u32 {
        match self {
            Suit::Spades => 0x1F0A0,
            Suit::Hearts => 0x1F0B0,
            Suit::Diamonds => 0x1F0C0,
            Suit::Clubs => 0x1F0D0,
        }
    }
}
