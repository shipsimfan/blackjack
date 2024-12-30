use crate::{messages::Generate, model::PlayerState};

impl Generate for PlayerState {
    fn generate(&self, output: &mut Vec<u8>) {
        match self {
            PlayerState::NotPlaying => 0u8,
            PlayerState::PlayingNextRound => 1,
            PlayerState::PlayingThisRound => 2,
        }
        .generate(output)
    }
}
