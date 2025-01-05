use crate::messages::{Generate, ShuffleServerMessage};

impl Generate for ShuffleServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        0u8.generate(output);
    }
}
