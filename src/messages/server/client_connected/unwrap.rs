use crate::{messages::ClientConnectedServerMessage, model::Player};

impl<'a> ClientConnectedServerMessage<'a> {
    /// Unwraps the [`Player`] contained into an owned version
    pub fn unwrap(self) -> Player {
        self.player.unwrap()
    }
}
