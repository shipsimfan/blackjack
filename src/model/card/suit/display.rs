use crate::model::Suit;

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
            Suit::Diamonds => "♦",
            Suit::Hearts => "♥",
        })
    }
}
