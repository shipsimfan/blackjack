use crate::Options;
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
    pub fn username(&self) -> Result<Username<'static>, ()> {
        Username::new(self.username.clone()).ok_or(todo!())
    }
}
