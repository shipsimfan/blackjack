use crate::{messages::Generate, model::Player};

impl Generate for Player {
    fn generate(&self, output: &mut Vec<u8>) {
        self.username.generate(output);
        self.ai.generate(output);
        self.state.generate(output);
        self.hands.generate(output);
    }
}

impl Generate for Option<Player> {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            Some(player) => {
                player.generate(output);
            }
            None => {
                0u8.generate(output);
            }
        }
    }
}
