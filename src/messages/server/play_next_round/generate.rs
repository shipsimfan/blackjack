use crate::messages::{Generate, PlayNextRoundServerMessage};

impl Generate for PlayNextRoundServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.client.generate(output);
        self.play_next_round.generate(output);
    }
}
