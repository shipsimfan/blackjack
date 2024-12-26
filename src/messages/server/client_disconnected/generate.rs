use crate::messages::{ClientDisconnectedServerMessage, Generate};

impl Generate for ClientDisconnectedServerMessage {
    fn generate(&self, output: &mut Vec<u8>) {
        self.id.generate(output);
    }
}
