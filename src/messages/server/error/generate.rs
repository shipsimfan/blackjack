use crate::messages::{ErrorServerMessage, Generate};

impl Generate for ErrorServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        (*self as u32).generate(output);
    }
}
