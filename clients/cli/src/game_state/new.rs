use super::Connecting;
use crate::{GameState, Options};

impl GameState {
    /// Creates a new [`GameState`] in the first state
    pub fn new(options: Options) -> GameState {
        GameState::Connecting(Connecting::new(options))
    }
}
