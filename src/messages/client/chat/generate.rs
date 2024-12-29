use crate::messages::{ChatClientMessage, Generate};

impl<'a> Generate for ChatClientMessage<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        self.message.generate(output);
    }
}
