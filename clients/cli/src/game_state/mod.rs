use connecting::Connecting;

mod connecting;
mod new;

/// The current state of program
pub enum GameState {
    /// Currently in the process of connecting to the server
    Connecting(Connecting),
}
