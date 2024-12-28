use crate::virtual_terminal::TextInput;
use blackjack::messages::Username;

mod get;
mod handle_message;
mod handle_terminal;
mod new;

/// A password must be entered to connect
pub struct PasswordEntryState {
    /// The username to present as
    username: Username<'static>,

    /// The input for password
    password_input: TextInput,

    /// The name of the server
    pub server_name: String,
}
