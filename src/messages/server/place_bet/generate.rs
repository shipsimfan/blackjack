use crate::messages::{Generate, PlaceBetServerMessage};

impl Generate for PlaceBetServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.client.generate(output);
        self.bet.generate(output);
    }
}
