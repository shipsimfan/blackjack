mod handle_message;
mod new;

/// Sends the client hello an wait for a game state response
pub struct WaitForGameState {
    /// The name of the server
    pub server_name: String,
}
