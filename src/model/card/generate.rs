use crate::{
    messages::Generate,
    model::{Card, Suit},
};

impl Generate for Card {
    fn generate(&self, output: &mut Vec<u8>) {
        (self.suit as u8 + self.rank as u8 * Suit::ALL.len() as u8).generate(output);
    }
}
