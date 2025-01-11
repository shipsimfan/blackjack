use crate::messages::{Generate, HitServerMessage};

impl Generate for HitServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.card.generate(output);
    }
}
