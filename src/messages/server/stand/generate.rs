use crate::messages::{Generate, StandServerMessage};

impl Generate for StandServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        0u8.generate(output);
    }
}
