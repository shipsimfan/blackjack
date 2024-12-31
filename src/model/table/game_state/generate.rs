use crate::{messages::Generate, model::GameState};

impl Generate for GameState {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            GameState::WaitingForPlayers => 0u8.generate(output),
            GameState::WaitingForBets => 1u8.generate(output),
            GameState::WaitingForPlayer(player, hand) => {
                2u8.generate(output);
                player.generate(output);
                hand.generate(output);
            }
        }
    }
}
