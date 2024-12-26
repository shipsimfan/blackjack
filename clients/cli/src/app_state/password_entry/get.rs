use super::PasswordEntryState;
use blackjack::messages::Username;

impl PasswordEntryState {
    /// Gets the username to present as
    pub fn username(&self) -> Username {
        self.username.borrow()
    }

    /// Gets the password that is currently entered
    pub fn password(&self) -> &str {
        self.password_input.value()
    }
}
