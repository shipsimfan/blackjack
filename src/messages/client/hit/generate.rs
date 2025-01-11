use crate::messages::{Generate, HitClientMessage};

impl Generate for HitClientMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        0u8.generate(output);
    }
}
