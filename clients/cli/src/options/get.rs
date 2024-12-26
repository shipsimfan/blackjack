use crate::{InvalidUsernameError, Options};
use blackjack::messages::Username;

impl Options {
    /// Gets the requested address to connect to
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Gets the port to connect to
    pub fn port(&self) -> u16 {
        self.port.get()
    }

    /// Gets the username to present to other players
    pub fn username(self) -> Result<Username<'static>, InvalidUsernameError> {
        Username::new(self.username.clone()).ok_or_else(|| InvalidUsernameError::new(self.username))
    }
}
