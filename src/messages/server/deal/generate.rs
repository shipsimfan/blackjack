use crate::messages::{DealServerMessage, Generate};

impl Generate for DealServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.dealer_up_card.generate(output);
        match self.dealer_down_card {
            Some(card) => card.generate(output),
            None => 0u8.generate(output),
        }
        self.hands.generate(output);
    }
}
