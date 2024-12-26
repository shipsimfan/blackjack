use super::Connecting;
use blackjack::messages::Username;

impl Connecting {
    /// Unwraps the username from this state
    pub fn unwrap(self) -> Username<'static> {
        self.username
    }
}
