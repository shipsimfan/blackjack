use crate::{messages::Generate, model::Hand};

impl Generate for Hand {
    fn generate(&self, output: &mut Vec<u8>) {
        self.bet.generate(output);
        self.cards.generate(output);
        self.hidden_card.generate(output);
    }
}

impl Hand {
    /// Generates the bytes to describe a dealer hand
    pub fn generate_dealer(&self, output: &mut Vec<u8>) {
        self.bet.generate(output);

        if self.hidden_card {
            self.cards[1..].generate(output);
            true.generate(output);
        } else {
            self.cards.generate(output);
            false.generate(output);
        }
    }
}
