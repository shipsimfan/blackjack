use crate::messages::{EndRoundServerMessage, Generate};

impl Generate for EndRoundServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.dealer_play.generate(output);
    }
}
