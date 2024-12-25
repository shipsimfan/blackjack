use super::Connecting;
use crate::{AppState, Options};

impl AppState {
    /// Creates a new [`AppState`] in the first state
    pub fn new(options: Options) -> AppState {
        AppState::Connecting(Connecting::new(options))
    }
}
