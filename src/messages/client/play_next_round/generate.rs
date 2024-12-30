use crate::messages::{Generate, PlayNextRoundClientMessage};

impl Generate for PlayNextRoundClientMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.play_next_round.generate(output);
    }
}
