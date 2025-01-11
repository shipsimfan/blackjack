use crate::messages::{Generate, StandClientMessage};

impl Generate for StandClientMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        0u8.generate(output);
    }
}
