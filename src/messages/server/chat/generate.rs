use crate::messages::{ChatServerMessage, Generate};

impl<'a> Generate for ChatServerMessage<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        self.client.generate(output);
        self.message.generate(output);
    }
}
