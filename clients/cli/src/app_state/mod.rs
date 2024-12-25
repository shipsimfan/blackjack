use connecting::Connecting;
use password_entry::PasswordEntryState;

mod connecting;
mod password_entry;

mod handle_message;
mod handle_terminal;
mod new;

/// The current state of program
pub enum AppState {
    /// Currently in the process of connecting to the server
    Connecting(Connecting),

    /// A password was required by the server and must be entered
    PasswordEntry(PasswordEntryState),
}
