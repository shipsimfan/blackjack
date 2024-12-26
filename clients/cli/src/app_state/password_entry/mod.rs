use crate::{virtual_terminal::TextInput, Options};

mod handle_message;
mod handle_terminal;
mod new;

/// A password must be entered to connect
pub struct PasswordEntryState {
    /// The options passed to the program
    options: Options,

    /// The input for password
    password_input: TextInput,
}
