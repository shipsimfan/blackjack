mod generate;
mod new;
mod parse;
mod to_static;

/// A client has disconnected from the server
#[derive(Debug, Clone)]
pub struct ClientDisconnectedServerMessage {
    /// The id of the disconnecting client
    pub id: u8,
}
