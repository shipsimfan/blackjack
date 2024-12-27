use crate::messages::{ClientConnectedServerMessage, Generate};

impl<'a> Generate for ClientConnectedServerMessage<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        self.player.generate(output);
    }
}
