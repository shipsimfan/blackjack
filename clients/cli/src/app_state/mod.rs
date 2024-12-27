use crate::MainGame;
use connecting::Connecting;
use password_entry::PasswordEntryState;
use wait_for_game_state::WaitForGameState;

mod connecting;
mod password_entry;
mod wait_for_game_state;

mod handle_message;
mod handle_terminal;
mod new;

/// The current state of program
pub enum AppState {
    /// Currently in the process of connecting to the server
    Connecting(Connecting),

    /// A password was required by the server and must be entered
    PasswordEntry(PasswordEntryState),

    /// Waiting for the server to respond with a game state
    WaitForGameState(WaitForGameState),

    /// Connected and playing
    MainGame(MainGame),
}
