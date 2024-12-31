use crate::model::Card;
use std::fmt::Write;

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from_u32(self.suit.unicode_base() + self.rank as u32).unwrap())
    }
}
