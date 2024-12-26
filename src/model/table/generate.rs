use crate::{messages::Generate, model::BlackjackTable};

impl Generate for BlackjackTable {
    fn generate(&self, output: &mut Vec<u8>) {
        self.players.generate(output);
    }
}
