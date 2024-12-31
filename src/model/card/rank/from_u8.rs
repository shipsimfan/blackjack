use crate::model::Rank;

impl Rank {
    /// Gets the rank from `byte`
    pub fn from_u8(byte: u8) -> Option<Self> {
        Some(match byte {
            0x1 => Rank::Ace,
            0x2 => Rank::Two,
            0x3 => Rank::Three,
            0x4 => Rank::Four,
            0x5 => Rank::Five,
            0x6 => Rank::Six,
            0x7 => Rank::Seven,
            0x8 => Rank::Eight,
            0x9 => Rank::Nine,
            0xA => Rank::Ten,
            0xB => Rank::Jack,
            0xD => Rank::Queen,
            0xE => Rank::King,
            _ => return None,
        })
    }
}
