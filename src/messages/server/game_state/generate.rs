use crate::messages::{GameStateServerMessage, Generate};

impl<'a> Generate for GameStateServerMessage<'a> {
    fn generate(&self, output: &mut Vec<u8>) {
        self.client_id.generate(output);
        self.table.generate(output);
    }
}
