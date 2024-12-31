use crate::model::Suit;

impl Suit {
    /// Gets the suit from `byte`
    pub fn from_u8(byte: u8) -> Option<Self> {
        Some(match byte {
            0 => Suit::Clubs,
            1 => Suit::Spades,
            2 => Suit::Diamonds,
            3 => Suit::Hearts,
            _ => return None,
        })
    }
}
