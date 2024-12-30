use crate::{messages::PlayNextRoundServerMessage, model::PlayerState};

impl PlayNextRoundServerMessage {
    /// Gets the desired [`PlayerState`]
    pub fn as_state(&self) -> PlayerState {
        if self.play_next_round {
            PlayerState::PlayingNextRound
        } else {
            PlayerState::NotPlaying
        }
    }
}
