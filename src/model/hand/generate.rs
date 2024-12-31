use crate::{messages::Generate, model::Hand};

impl Generate for Hand {
    fn generate(&self, output: &mut Vec<u8>) {
        self.bet.generate(output);
    }
}
