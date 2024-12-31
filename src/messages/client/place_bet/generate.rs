use crate::messages::{Generate, PlaceBetClientMessage};

impl Generate for PlaceBetClientMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.bet.generate(output);
    }
}
